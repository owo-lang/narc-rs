use voile_util::tags::Plicit;
use voile_util::uid::{next_uid, DBI};

use crate::syntax::common::Bind;

use super::super::{Closure, Elim, Term, Val};
use super::{def_app, Subst};

/// Reducible expressions.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#Subst).
pub trait RedEx<T: Sized = Term>: Sized {
    /// Apply a substitution to a redex.
    fn reduce_dbi(self, subst: &Subst) -> T;

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.DeBruijn.html#DeBruijn).
    fn from_dbi(dbi: DBI) -> Self;
}

impl RedEx for Term {
    fn reduce_dbi(self, subst: &Subst) -> Term {
        match self {
            Term::Whnf(n) => n.reduce_dbi(subst),
            Term::Redex(f, id, args) => def_app(f, id, vec![], reduce_vec_dbi(args, &subst)),
        }
    }

    fn from_dbi(dbi: DBI) -> Self {
        Term::Whnf(RedEx::from_dbi(dbi))
    }
}

impl RedEx<Elim> for Elim {
    fn reduce_dbi(self, subst: &Subst) -> Elim {
        match self {
            Elim::App(term) => Elim::app(term.reduce_dbi(subst)),
            e => e,
        }
    }

    fn from_dbi(dbi: DBI) -> Self {
        Elim::app(RedEx::from_dbi(dbi))
    }
}

impl<R, T: RedEx<R>> RedEx<Bind<R>> for Bind<T> {
    fn reduce_dbi(self, subst: &Subst) -> Bind<R> {
        Bind::new(self.licit, self.name, self.ty.reduce_dbi(subst))
    }

    fn from_dbi(dbi: DBI) -> Self {
        eprintln!("Warning: generating new UID that is not resolved!");
        Bind::new(Plicit::Ex, unsafe { next_uid() }, RedEx::from_dbi(dbi))
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

    fn from_dbi(dbi: DBI) -> Self {
        Val::Var(dbi, vec![])
    }
}

impl RedEx<Closure> for Closure {
    fn reduce_dbi(self, subst: &Subst) -> Self {
        use Closure::*;
        let Plain(body) = self;
        Self::plain(body.reduce_dbi(subst))
    }

    fn from_dbi(dbi: DBI) -> Self {
        Self::plain(RedEx::from_dbi(dbi))
    }
}

fn reduce_vec_dbi<T>(me: Vec<impl RedEx<T>>, subst: &Subst) -> Vec<T> {
    me.into_iter().map(|e| e.reduce_dbi(subst)).collect()
}
