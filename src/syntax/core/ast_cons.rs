use voile_util::level::Level;
use voile_util::meta::MI;
use voile_util::tags::{Plicit, VarRec};
use voile_util::uid::*;

use crate::syntax::core::{Closure, Elim, Term, Val};

/// Constructors and traversal functions.
impl Term {
    pub fn is_type(&self) -> bool {
        use Val::*;
        match match self {
            Term::Whnf(val) => val,
            Term::Redex(..) => return false,
        } {
            Id(..) | Type(..) | Pi(..) | Data(..) => true,
            // In case it's neutral, we use `is_universe` on its type.
            // In case it's a meta, we're supposed to solve it.
            Refl | App(..) | Meta(..) | Cons(..) | Axiom(..) => false,
        }
    }

    pub fn is_universe(&self) -> bool {
        match self {
            Term::Whnf(Val::Type(..)) => true,
            _ => false,
        }
    }

    pub fn cons(name: String, params: Vec<Term>) -> Self {
        Term::Whnf(Val::Cons(name, params))
    }

    pub fn data(kind: VarRec, params: Vec<Term>) -> Self {
        Term::Whnf(Val::Data(kind, params))
    }

    pub fn inductive(params: Vec<Term>) -> Self {
        Self::data(VarRec::Variant, params)
    }

    pub fn coinductive(params: Vec<Term>) -> Self {
        Self::data(VarRec::Record, params)
    }

    pub fn meta(index: MI, params: Vec<Elim>) -> Self {
        Term::Whnf(Val::Meta(index, params))
    }

    pub fn reflexivity() -> Self {
        Term::Whnf(Val::Refl)
    }

    pub fn universe(level: Level) -> Self {
        Term::Whnf(Val::Type(level))
    }

    pub fn identity(ty: Self, a: Self, b: Self) -> Self {
        Term::Whnf(Val::Id(Box::new(ty), Box::new(a), Box::new(b)))
    }

    pub fn fresh_axiom() -> Self {
        Self::postulate(unsafe { next_uid() })
    }

    pub(crate) fn postulate(uid: UID) -> Self {
        Term::Whnf(Val::Axiom(uid))
    }

    pub fn pi(plicit: Plicit, param_type: Term, body: Closure) -> Term {
        Term::Whnf(Val::Pi(plicit, Box::new(param_type), body))
    }
}

impl Closure {
    pub fn plain(body: Term) -> Self {
        Closure::Plain(Box::new(body))
    }
}
