use voile_util::level::Level;
use voile_util::loc::Ident;
use voile_util::meta::MI;
use voile_util::tags::VarRec;
use voile_util::uid::{DBI, GI, UID};

use crate::syntax::common::{self, Ductive};

use super::subst::{RedEx, Subst};

/// Constructor information.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Internal.html#ConHead).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConHead {
    /// Constructor name.
    pub name: Ident,
    /// Index of the constructor.
    pub cons_ix: GI,
    /// Records might be coinductive.
    pub ductive: Ductive,
    /// Field names.
    /// This allows us to project fields from a record without the `TCS`.
    pub fields: Vec<String>,
}

impl ConHead {
    pub fn pseudo(name: Ident) -> Self {
        Self::new(name, Default::default(), Ductive::In, vec![])
    }

    pub fn new(name: Ident, ix: GI, ductive: Ductive, fields: Vec<String>) -> Self {
        Self {
            name,
            cons_ix: ix,
            ductive,
            fields,
        }
    }
}

/// Weak-head-normal-form terms, canonical values.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Val {
    /// Type universe.
    Type(Level),
    /// (Co)Data types, fully applied.
    Data(VarRec, GI, Vec<Term>),
    /// Pi-like types (dependent types), with parameter explicitly typed.
    Pi(Bind<Box<Term>>, Closure),
    /// Constructor invocation, fully applied.
    Cons(ConHead, Vec<Term>),
    /// Meta reference, with eliminations.
    /// This does not appear in Cockx18, but we can find it in the
    /// [implementation](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Internal.html#v:MetaV).
    Meta(MI, Vec<Elim>),
    /// Postulated values.
    Axiom(UID),
    /// Variable elimination, in spine-normal form.
    /// (so we have easy access to application arguments).<br/>
    /// This is convenient for meta resolution and termination check.
    Var(DBI, Vec<Elim>),
    /// The homogeneous identity (equality) type.
    /// Arguments are the type and two inhabitants.
    Id(Box<Term>, Box<Term>, Box<Term>),
    /// Proof of reflexivity.
    Refl,
}

/// Type for terms.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term {
    Whnf(Val),
    Redex(GI, Ident, Vec<Elim>),
}

pub type Bind<T = Term> = common::Bind<T>;
pub type Let<T = Term> = common::Let<T>;

/// Type for eliminations.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Elim {
    App(Box<Term>),
    Proj(String),
}

/// A closure with open terms.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Closure {
    Plain(Box<Term>),
}

impl Closure {
    pub fn instantiate(self, arg: Term) -> Term {
        self.instantiate_safe(arg)
            .unwrap_or_else(|e| panic!("Cannot split on `{}`.", e))
    }

    pub fn instantiate_safe(self, arg: Term) -> Result<Term, Term> {
        let Closure::Plain(body) = self;
        Ok(body.reduce_dbi(Subst::one(arg)))
    }
}
