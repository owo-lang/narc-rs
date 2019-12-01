use std::rc::Rc;

use voile_util::uid::DBI;

use crate::syntax::core::{Term, Val};

use super::PrimSubst;

pub trait DeBruijn {
    fn dbi_view(&self) -> Option<DBI>;
}

impl DeBruijn for Val {
    fn dbi_view(&self) -> Option<DBI> {
        match self {
            Val::Var(i, v) if v.is_empty() => Some(*i),
            _ => None,
        }
    }
}

impl DeBruijn for Term {
    fn dbi_view(&self) -> Option<DBI> {
        match self {
            Term::Whnf(w) => w.dbi_view(),
            Term::Redex(..) => None,
        }
    }
}

impl<T: DeBruijn> PrimSubst<T> {
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#%2B%2B%23).
    pub fn concat(ts: impl Iterator<Item = T>, to: Rc<Self>) -> Rc<Self> {
        ts.fold(to, |to, t| Self::cons(t, to))
    }

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#consS).
    pub fn cons(t: T, rho: Rc<Self>) -> Rc<Self> {
        match (t.dbi_view(), &*rho) {
            (Some(n), PrimSubst::Weak(m, rho)) if n + 1 == *m => {
                Self::weaken(Self::lift_by(rho.clone(), DBI(1)), *m - 1)
            }
            _ => Rc::new(PrimSubst::Cons(t, rho)),
        }
    }
}
