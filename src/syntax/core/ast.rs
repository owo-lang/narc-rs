use std::collections::BTreeMap;

use voile_util::level::Level;
use voile_util::meta::MI;
use voile_util::tags::{PiSig, Plicit};
use voile_util::uid::{DBI, GI};

use super::{RedEx, TraverseNeutral};
use voile_util::axiom::Axiom;

/// Case-split expression.
pub type CaseSplit = BTreeMap<String, Closure>;

/// Reduction functions.
impl Val {
    pub fn apply(self, arg: Val) -> Self {
        match self {
            Val::Lam(closure) => closure.instantiate(arg),
            Val::Neut(Neutral::App(f, mut a)) => {
                a.push(arg);
                Val::app(*f, a)
            }
            Val::Neut(otherwise) => Val::app(otherwise, vec![arg]),
            e => panic!("Cannot apply on `{}`.", e),
        }
    }

    pub fn first(self) -> Self {
        match self {
            Val::Pair(a, _) => *a,
            Val::Neut(otherwise) => Val::fst(otherwise),
            e => panic!("Cannot project on `{}`.", e),
        }
    }

    pub fn second(self) -> Val {
        match self {
            Val::Pair(_, b) => *b,
            Val::Neut(otherwise) => Val::snd(otherwise),
            e => panic!("Cannot project on `{}`.", e),
        }
    }

    pub(crate) fn attach_dbi(self, dbi: DBI) -> Self {
        self.map_neutral(&mut |neut: Neutral| {
            Val::Neut(neut.map_axiom(&mut |a| {
                Neutral::Axi(match a {
                    Axiom::Postulated(uid) => Axiom::Generated(uid, dbi),
                    e => e,
                })
            }))
        })
    }

    pub fn generated_to_var(self) -> Self {
        use {Axiom::*, Neutral::*};
        self.map_axiom(&mut |a| match a {
            Postulated(..) | Unimplemented(..) | Implicit(..) => Axi(a),
            Generated(_, dbi) => Var(dbi),
        })
    }

    pub fn unimplemented_to_glob(self) -> Self {
        use {Axiom::*, Neutral::*};
        self.map_axiom(&mut |a| match a {
            Postulated(..) | Generated(..) | Implicit(..) => Axi(a),
            Unimplemented(_, dbi) => Ref(dbi),
        })
    }

    pub fn map_axiom(self, f: &mut impl FnMut(Axiom) -> Neutral) -> Self {
        self.map_neutral(&mut |neut| Val::Neut(neut.map_axiom(f)))
    }
}

/// Irreducible because of the presence of generated value.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Neutral {
    /// Local variable, referred by de-bruijn index.
    Var(DBI),
    /// Global variable, referred by index. Needed for recursive definitions.
    Ref(GI),
    /// Meta variable reference.
    Meta(MI),
    /// Lifting self to a higher/lower level.
    Lift(u32, Box<Self>),
    /// Postulated value, aka axioms.
    Axi(Axiom),
    /// Function application, with all arguments collected
    /// (so we have easy access to application arguments).<br/>
    /// This is convenient for meta resolution and termination check.
    ///
    /// The "arguments" is supposed to be non-empty.
    App(Box<Self>, Vec<Val>),
    /// Projecting the first element of a pair.
    Fst(Box<Self>),
    /// Projecting the second element of a pair.
    Snd(Box<Self>),
    /// Splitting on a neutral term.
    SplitOn(CaseSplit, Box<Self>),
}

impl Neutral {
    pub fn map_axiom(self, f: &mut impl FnMut(Axiom) -> Neutral) -> Self {
        use Neutral::*;
        let mapper = &mut |n: Neutral| Val::Neut(n.map_axiom(f));
        match self {
            Axi(a) => f(a),
            App(fun, args) => App(
                Box::new(fun.map_axiom(f)),
                args.into_iter()
                    .map(|a| a.map_neutral(&mut |n| Val::Neut(n.map_axiom(f))))
                    .collect(),
            ),
            Fst(p) => Fst(Box::new(p.map_axiom(f))),
            Snd(p) => Snd(Box::new(p.map_axiom(f))),
            Var(n) => Var(n),
            Ref(n) => Ref(n),
            Meta(n) => Meta(n),
            Lift(levels, expr) => Lift(levels, Box::new(expr.map_axiom(f))),
            SplitOn(split, obj) => SplitOn(
                Self::map_axiom_split(mapper, split),
                Box::new(obj.map_axiom(f)),
            ),
        }
    }

    fn map_axiom_split(mapper: &mut impl FnMut(Neutral) -> Val, split: CaseSplit) -> CaseSplit {
        split
            .into_iter()
            .map(|(k, v)| (k, v.map_neutral(mapper)))
            .collect()
    }
}

/// Type values.
pub type TVal = Val;

/// Non-redex, canonical values.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Val {
    /// Type universe.
    Type(Level),
    /// Closure with parameter typed.
    /// For untyped closures, it can be represented as `Neut` directly.
    Lam(Closure),
    /// Pi-like types (dependent types), with parameter explicitly typed.
    Dt(PiSig, Plicit, Box<Self>, Closure),
    /// Constructor invocation.
    Cons(String, Box<Self>),
    /// Sigma instance.
    Pair(Box<Self>, Box<Self>),
    /// Neutral value means irreducible but not canonical values.
    Neut(Neutral),
}

impl Default for Val {
    fn default() -> Self {
        Self::fresh_axiom()
    }
}

/// A closure with parameter type explicitly specified.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Closure {
    Plain(Box<Val>),
    Tree(CaseSplit),
}

impl Default for Closure {
    fn default() -> Self {
        Closure::Tree(Default::default())
    }
}

impl Closure {
    pub fn instantiate(self, arg: Val) -> Val {
        self.instantiate_safe(arg)
            .unwrap_or_else(|e| panic!("Cannot split on `{}`.", e))
    }

    pub fn instantiate_safe(self, arg: Val) -> Result<Val, Val> {
        match self {
            Closure::Plain(body) => Ok(body.reduce_with_dbi(arg, Default::default())),
            Closure::Tree(mut split) => match arg {
                Val::Cons(label, arg) => match split.remove(&label) {
                    Some(body) => body.instantiate_safe(*arg),
                    None => Err(Val::Cons(label, arg)),
                },
                Val::Neut(neutral) => Ok(Val::split_on(split, neutral)),
                a => Err(a),
            },
        }
    }

    pub fn instantiate_cloned(&self, arg: Val) -> Val {
        match self {
            Closure::Plain(body) => body.clone().reduce_with_dbi(arg, Default::default()),
            Closure::Tree(split) => match arg {
                Val::Cons(label, arg) => match split.get(&label) {
                    Some(body) => body.instantiate_cloned(*arg),
                    None => panic!("Cannot find clause for label `{}`.", label),
                },
                Val::Neut(neutral) => Val::split_on(split.clone(), neutral),
                a => panic!("Cannot split on `{}`.", a),
            },
        }
    }

    pub fn instantiate_borrow(&self, arg: &Val) -> Val {
        match self {
            Closure::Plain(body) => body.clone().reduce_with_dbi_borrow(arg, Default::default()),
            Closure::Tree(split) => match arg {
                Val::Cons(label, arg) => match split.get(label) {
                    Some(body) => body.instantiate_borrow(arg),
                    None => panic!("Cannot find clause for label `{}`.", label),
                },
                Val::Neut(neutral) => Val::split_on(split.clone(), neutral.clone()),
                a => panic!("Cannot split on `{}`.", a),
            },
        }
    }
}
