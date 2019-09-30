use voile_util::level::Level;
use voile_util::meta::MI;
use voile_util::tags::{Plicit, VarRec};
use voile_util::uid::*;

use crate::syntax::core::{Bind, Tele};

use super::{Closure, ConHead, Elim, Term, Val};

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
            Refl | Var(..) | Meta(..) | Cons(..) | Axiom(..) => false,
        }
    }

    pub fn is_universe(&self) -> bool {
        match self {
            Term::Whnf(Val::Type(..)) => true,
            _ => false,
        }
    }

    pub fn cons(name: ConHead, params: Vec<Term>) -> Self {
        Term::Whnf(Val::Cons(name, params))
    }

    pub fn data(kind: VarRec, ix: GI, params: Vec<Term>) -> Self {
        Term::Whnf(Val::Data(kind, ix, params))
    }

    pub fn inductive(ix: GI, params: Vec<Term>) -> Self {
        Self::data(VarRec::Variant, ix, params)
    }

    pub fn coinductive(ix: GI, params: Vec<Term>) -> Self {
        Self::data(VarRec::Record, ix, params)
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

    pub fn def(gi: GI, elims: Vec<Elim>) -> Self {
        Term::Redex(gi, elims)
    }

    pub fn simple_def(gi: GI) -> Self {
        Self::def(gi, vec![])
    }

    pub fn pi_from_tele(tele: Tele, ret: Self) -> Self {
        tele.into_iter().rfold(ret, |ret, param| {
            Self::pi2(param.map_term(Box::new), Closure::plain(ret))
        })
    }

    pub fn pi(licit: Plicit, name: UID, param_type: Term, body: Closure) -> Self {
        Self::pi2(Bind::boxing(licit, name, param_type), body)
    }

    pub fn pi2(param: Bind<Box<Term>>, body: Closure) -> Self {
        Term::Whnf(Val::Pi(param, body))
    }
}

impl Closure {
    pub fn plain(body: Term) -> Self {
        Closure::Plain(Box::new(body))
    }
}

impl Elim {
    pub fn app(term: Term) -> Self {
        Elim::App(Box::new(term))
    }

    pub fn into_app(self) -> Term {
        self.try_into_app().unwrap()
    }

    pub fn try_into_app(self) -> Result<Term, String> {
        match self {
            Elim::App(term) => Ok(*term),
            Elim::Proj(field) => Err(field),
        }
    }
}
