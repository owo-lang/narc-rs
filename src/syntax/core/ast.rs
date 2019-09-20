use voile_util::level::Level;
use voile_util::meta::MI;
use voile_util::tags::{Plicit, VarRec};
use voile_util::uid::{DBI, GI, UID};

use crate::syntax::pat;

pub type Pat = pat::Copat<DBI, Term>;

/// Reduction functions.
impl Val {
    pub fn eliminate(self, arg: Elim) -> Self {
        match self {
            Val::App(f, mut a) => {
                a.push(arg);
                Val::App(f, a)
            }
            Val::Meta(m, mut a) => {
                a.push(arg);
                Val::Meta(m, a)
            }
            e => panic!("Cannot eliminate `{}`.", e),
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
    Pi(Plicit, Box<Term>, Closure),
    /// Constructor invocation, fully applied.
    Cons(String, Vec<Term>),
    /// Meta reference, with eliminations.
    /// This does not appear in Cockx18, but we can find it in the
    /// [implementation](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Internal.html#v:MetaV).
    Meta(MI, Vec<Elim>),
    /// Postulated values.
    Axiom(UID),
    /// Variable elimination, in spine-normal form.
    /// (so we have easy access to application arguments).<br/>
    /// This is convenient for meta resolution and termination check.
    App(DBI, Vec<Elim>),
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
    Redex(GI, Vec<Elim>),
}

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
