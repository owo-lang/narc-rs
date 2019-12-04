use std::rc::Rc;

use either::Either;
use voile_util::uid::DBI;

use crate::syntax::common::Bind;
use crate::syntax::pat::{Copat, Pat};

use super::super::{Closure, Elim, Term, Val};
use super::{def_app, DeBruijn, PrimSubst, Subst};

/// Reducible expressions.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#Subst).
pub trait RedEx<T: Sized = Self>: Sized {
    /// Apply a substitution to a redex.
    fn reduce_dbi(self, subst: &Subst) -> T;
}

impl RedEx for Term {
    fn reduce_dbi(self, subst: &Subst) -> Term {
        match self {
            Term::Whnf(n) => n.reduce_dbi(subst),
            Term::Redex(f, id, args) => def_app(f, id, vec![], args.reduce_dbi(&subst)),
        }
    }
}

impl RedEx for Elim {
    fn reduce_dbi(self, subst: &Subst) -> Elim {
        match self {
            Elim::App(term) => Elim::app(term.reduce_dbi(subst)),
            e => e,
        }
    }
}

impl<R, T: RedEx<R>> RedEx<Bind<R>> for Bind<T> {
    fn reduce_dbi(self, subst: &Subst) -> Bind<R> {
        self.map_term(|t| t.reduce_dbi(subst), |v| Some(v.reduce_dbi(subst)))
    }
}

impl RedEx<Term> for Val {
    fn reduce_dbi(self, subst: &Subst) -> Term {
        let reduce_vec = |a: Vec<Term>| a.into_iter().map(|a| a.reduce_dbi(&subst)).collect();
        match self {
            Val::Pi(arg, closure) => Term::pi(
                arg.licit,
                arg.name,
                arg.ty.reduce_dbi(subst),
                closure.reduce_dbi(subst),
            ),
            Val::Cons(name, a) => Term::cons(name, reduce_vec(a)),
            Val::Type(n) => Term::universe(n),
            Val::Data(kind, gi, a) => Term::data(kind, gi, reduce_vec(a)),
            Val::Meta(m, a) => Term::meta(m, a.reduce_dbi(&subst)),
            Val::Var(f, args) => subst.lookup(f).apply_elim(args.reduce_dbi(subst)),
            Val::Axiom(a) => Term::Whnf(Val::Axiom(a)),
            Val::Refl => Term::reflexivity(),
            Val::Id(ty, a, b) => Term::identity(
                ty.reduce_dbi(subst),
                a.reduce_dbi(subst),
                b.reduce_dbi(subst),
            ),
        }
    }
}

impl RedEx for Closure {
    fn reduce_dbi(self, subst: &Subst) -> Self {
        use Closure::*;
        let Plain(body) = self;
        Self::plain(body.reduce_dbi(subst))
    }
}

impl<R, T: RedEx<R>> RedEx<Vec<R>> for Vec<T> {
    fn reduce_dbi(self, subst: &Subst) -> Vec<R> {
        self.into_iter().map(|e| e.reduce_dbi(subst)).collect()
    }
}

impl<Ix, R, T: RedEx<R>> RedEx<Copat<Ix, R>> for Copat<Ix, T> {
    fn reduce_dbi(self, subst: &Subst) -> Copat<Ix, R> {
        match self {
            Copat::App(a) => Copat::App(a.reduce_dbi(subst)),
            Copat::Proj(p) => Copat::Proj(p),
        }
    }
}

impl<Ix, R, T: RedEx<R>> RedEx<Pat<Ix, R>> for Pat<Ix, T> {
    fn reduce_dbi(self, subst: &Subst) -> Pat<Ix, R> {
        match self {
            Pat::Refl => Pat::Refl,
            Pat::Absurd => Pat::Absurd,
            Pat::Var(v) => Pat::Var(v),
            Pat::Cons(f, c, pats) => Pat::Cons(f, c, pats.reduce_dbi(subst)),
            Pat::Forced(t) => Pat::Forced(t.reduce_dbi(subst)),
        }
    }
}

impl PrimSubst<Term> {
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#composeS).
    pub fn compose(rho: Rc<Self>, sgm: Rc<Self>) -> Rc<Self> {
        use PrimSubst::*;
        match (&*rho, &*sgm) {
            (_, IdS) => rho,
            (IdS, _) => sgm,
            // rho, EmptyS(err) => EmptyS(err)
            (_, Weak(n, sgm)) => Self::compose(Self::drop_by(rho, *n), sgm.clone()),
            (_, Cons(u, sgm)) => Rc::new(Cons(
                u.clone().reduce_dbi(&*rho),
                Self::compose(rho, sgm.clone()),
            )),
            (_, Succ(sgm)) => Rc::new(Succ(Self::compose(rho, sgm.clone()))),
            (_, Lift(DBI(0), _sgm)) => unreachable!(),
            (Cons(u, rho), Lift(n, sgm)) => Rc::new(Cons(
                u.clone(),
                Self::compose(rho.clone(), Self::lift_by(sgm.clone(), *n - 1)),
            )),
            (_, Lift(n, sgm)) => Rc::new(Cons(
                rho.lookup(DBI(0)),
                Self::compose(
                    rho.clone(),
                    Self::weaken(Self::lift_by(sgm.clone(), *n - 1), DBI(1)),
                ),
            )),
        }
    }

    /// If lookup failed, return the DBI.
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#lookupS).
    pub fn lookup_impl(&self, dbi: DBI) -> Either<&Term, Term> {
        use Either::*;
        use PrimSubst::*;
        match self {
            IdS => Right(Term::from_dbi(dbi)),
            Cons(o, rest) => match dbi.nat() {
                None => Left(o),
                Some(dbi) => rest.lookup_impl(dbi),
            },
            Succ(rest) => rest.lookup_impl(dbi.pred()),
            Weak(i, rest) => match &**rest {
                IdS => Right(Term::from_dbi(dbi + *i)),
                rho => Right(rho.lookup(*i).reduce_dbi(&*Self::raise(*i))),
            },
            Lift(n, _) if dbi < *n => Right(Term::from_dbi(dbi)),
            Lift(n, rest) => Right(Self::raise_term(*n, rest.lookup(dbi - *n))),
        }
    }
}
