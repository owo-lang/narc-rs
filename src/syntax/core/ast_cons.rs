use voile_util::axiom::Axiom;
use voile_util::meta::MI;
use voile_util::tags::Plicit;
use voile_util::uid::*;

use crate::syntax::core::{CaseSplit, Closure, Neutral, TVal, Val};

/// Constructors and traversal functions.
impl Val {
    pub fn is_type(&self) -> bool {
        use Val::*;
        match self {
            Type(..) | Pi(..) => true,
            // In case it's neutral, we use `is_universe` on its type.
            // In case it's a meta, we're supposed to solve it.
            Lam(..) | Cons(..) | Neut(..) => false,
        }
    }

    pub fn is_universe(&self) -> bool {
        match self {
            Val::Type(..) => true,
            _ => false,
        }
    }

    pub fn cons(name: String, param: Self) -> Self {
        Val::Cons(name, Box::new(param))
    }

    pub fn case_tree(tree: CaseSplit) -> Self {
        Val::Lam(Closure::Tree(tree))
    }

    pub fn lift(levels: u32, expr: Neutral) -> Self {
        Val::Neut(Neutral::Lift(levels, Box::new(expr)))
    }

    pub fn meta(index: MI) -> Self {
        Val::Neut(Neutral::Meta(index))
    }

    pub fn var(index: DBI) -> Self {
        Val::Neut(Neutral::Var(index))
    }

    pub fn closure_lam(body: Self) -> Self {
        Val::Lam(Closure::plain(body))
    }

    pub fn glob(index: GI) -> Self {
        Val::Neut(Neutral::Ref(index))
    }

    pub fn split_on(split: CaseSplit, on: Neutral) -> Self {
        Val::Neut(Neutral::SplitOn(split, Box::new(on)))
    }

    pub fn fresh_axiom() -> Self {
        Self::postulate(unsafe { next_uid() })
    }

    pub(crate) fn postulate(uid: UID) -> Self {
        Val::Neut(Neutral::Axi(Axiom::Postulated(uid)))
    }

    pub fn fresh_implicit() -> Self {
        let axiom = Axiom::Implicit(unsafe { next_uid() });
        Val::Neut(Neutral::Axi(axiom))
    }

    pub fn fresh_unimplemented(index: GI) -> Self {
        let axiom = Axiom::Unimplemented(unsafe { next_uid() }, index);
        Val::Neut(Neutral::Axi(axiom))
    }

    pub fn app(function: Neutral, args: Vec<Self>) -> Self {
        Val::Neut(Neutral::App(Box::new(function), args))
    }

    pub fn pi(plicit: Plicit, param_type: TVal, body: Closure) -> TVal {
        Val::Pi(plicit, Box::new(param_type), body)
    }

    pub fn into_neutral(self) -> Result<Neutral, Self> {
        match self {
            Val::Neut(n) => Ok(n),
            e => Err(e),
        }
    }
}

impl Closure {
    pub fn plain(body: Val) -> Self {
        Closure::Plain(Box::new(body))
    }
}
