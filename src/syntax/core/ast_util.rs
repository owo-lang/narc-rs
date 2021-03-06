use voile_util::{
    level::Level,
    loc::Ident,
    meta::MI,
    tags::{Plicit, VarRec},
    uid::*,
};

use crate::syntax::{
    common::ConHead,
    core::{Bind, Closure, Elim, Tele, Term, Val, ValData},
};

pub const TYPE_OMEGA: Val = Val::Type(Level::Omega);

impl ValData {
    pub fn new(kind: VarRec, def: GI, args: Vec<Term>) -> Self {
        Self { kind, def, args }
    }
}

impl Val {
    pub fn inductive(ix: GI, params: Vec<Term>) -> Self {
        Val::Data(ValData::new(VarRec::Variant, ix, params))
    }

    pub fn identity(ty: Term, a: Term, b: Term) -> Self {
        Val::Id(Box::new(ty), Box::new(a), Box::new(b))
    }

    pub fn coinductive(ix: GI, params: Vec<Term>) -> Self {
        Val::Data(ValData::new(VarRec::Record, ix, params))
    }
}

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

    pub fn cons(name: ConHead, params: Vec<Self>) -> Self {
        Term::Whnf(Val::Cons(name, params))
    }

    pub fn data(info: ValData) -> Self {
        Term::Whnf(Val::Data(info))
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
        Term::Whnf(Val::identity(ty, a, b))
    }

    pub fn fresh_axiom() -> Self {
        Self::postulate(unsafe { next_uid() })
    }

    pub(crate) fn postulate(uid: UID) -> Self {
        Term::Whnf(Val::Axiom(uid))
    }

    pub fn def(gi: GI, ident: Ident, elims: Vec<Elim>) -> Self {
        Term::Redex(gi, ident, elims)
    }

    pub fn simple_def(gi: GI, ident: Ident) -> Self {
        Self::def(gi, ident, vec![])
    }

    pub fn pi_from_tele(tele: Tele, ret: Self) -> Self {
        tele.into_iter().rfold(ret, |ret, param| {
            Self::pi2(param.boxed(), Closure::plain(ret))
        })
    }

    /// # Returns
    ///
    /// The telescope and the return type.
    pub fn tele_view(self) -> (Tele, Self) {
        match self {
            Term::Whnf(Val::Pi(bind, Closure::Plain(r))) => {
                let (mut view, r) = r.tele_view();
                view.insert(0, bind.unboxed());
                (view, r)
            }
            // The capacity is an arbitrarily estimated value.
            e => (Vec::with_capacity(2), e),
        }
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

    pub fn is_proj(&self) -> bool {
        match self {
            Elim::App(..) => false,
            Elim::Proj(..) => true,
        }
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
