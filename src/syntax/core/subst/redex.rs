use std::rc::Rc;

use either::Either;
use voile_util::uid::DBI;

use crate::syntax::common::Bind;

use super::super::{Closure, Elim, Term, Val};
use super::{def_app, DeBruijn, PrimSubst, Subst};

/// Reducible expressions.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#Subst).
pub trait RedEx<T: Sized = Term>: Sized {
    /// Apply a substitution to a redex.
    fn reduce_dbi(self, subst: &Subst) -> T;
}

impl RedEx for Term {
    fn reduce_dbi(self, subst: &Subst) -> Term {
        match self {
            Term::Whnf(n) => n.reduce_dbi(subst),
            Term::Redex(f, id, args) => def_app(f, id, vec![], reduce_vec_dbi(args, &subst)),
        }
    }
}

impl RedEx<Elim> for Elim {
    fn reduce_dbi(self, subst: &Subst) -> Elim {
        match self {
            Elim::App(term) => Elim::app(term.reduce_dbi(subst)),
            e => e,
        }
    }
}

impl<R, T: RedEx<R>> RedEx<Bind<R>> for Bind<T> {
    fn reduce_dbi(self, subst: &Subst) -> Bind<R> {
        Bind::new(self.licit, self.name, self.ty.reduce_dbi(subst))
    }
}

impl RedEx for Val {
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
            Val::Meta(m, a) => Term::meta(m, reduce_vec_dbi(a, &subst)),
            Val::Var(f, args) => subst.lookup(f).apply_elim(reduce_vec_dbi(args, subst)),
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

impl RedEx<Closure> for Closure {
    fn reduce_dbi(self, subst: &Subst) -> Self {
        use Closure::*;
        let Plain(body) = self;
        Self::plain(body.reduce_dbi(subst))
    }
}

fn reduce_vec_dbi<T>(me: Vec<impl RedEx<T>>, subst: &Subst) -> Vec<T> {
    me.into_iter().map(|e| e.reduce_dbi(subst)).collect()
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
            Lift(n, rest) => {
                if dbi < *n {
                    Right(Term::from_dbi(dbi))
                } else {
                    Right(Self::raise_term(*n, rest.lookup(dbi - *n)))
                }
            }
        }
    }

    pub fn lookup(&self, dbi: DBI) -> Term {
        self.lookup_impl(dbi).map_left(Clone::clone).into_inner()
    }

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#raise).
    pub fn raise_term(k: DBI, term: Term) -> Term {
        Self::raise_from(DBI(0), k, term)
    }

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#raiseFrom).
    pub fn raise_from(n: DBI, k: DBI, term: Term) -> Term {
        term.reduce_dbi(&Self::lift_by(Self::raise(k), n))
    }
}
