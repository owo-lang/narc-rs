use voile_util::meta::MetaSolution;

use crate::syntax::common::{Bind, Let};
use crate::syntax::pat::{Copat, Pat};

use super::super::{Closure, Elim, Term, Val};
use super::{def_app, Subst};

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

impl<R, T: RedEx<R>> RedEx<MetaSolution<R>> for MetaSolution<T> {
    fn reduce_dbi(self, subst: &Subst) -> MetaSolution<R> {
        match self {
            MetaSolution::Solved(t) => MetaSolution::Solved(Box::new(t.reduce_dbi(subst))),
            MetaSolution::Inlined => MetaSolution::Inlined,
            MetaSolution::Unsolved => MetaSolution::Unsolved,
        }
    }
}

impl<R, T: RedEx<R>> RedEx<Bind<R>> for Bind<T> {
    fn reduce_dbi(self, subst: &Subst) -> Bind<R> {
        self.map_term(|t| t.reduce_dbi(subst))
    }
}

impl<R, T: RedEx<R>> RedEx<Let<R>> for Let<T> {
    fn reduce_dbi(self, subst: &Subst) -> Let<R> {
        Let::new(self.bind.reduce_dbi(subst), self.val.reduce_dbi(subst))
    }
}

impl RedEx<Term> for Val {
    fn reduce_dbi(self, subst: &Subst) -> Term {
        let reduce_vec = |a: Vec<Term>| a.into_iter().map(|a| a.reduce_dbi(&subst)).collect();
        match self {
            Val::Pi(arg, closure) => Term::pi2(
                arg.unboxed().reduce_dbi(subst).boxed(),
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

/// For `Tele`.
impl<R, T: RedEx<R>> RedEx<Vec<R>> for Vec<T> {
    fn reduce_dbi(self, subst: &Subst) -> Vec<R> {
        self.into_iter().map(|e| e.reduce_dbi(subst)).collect()
    }
}

impl<A, B, X: RedEx<A>, Y: RedEx<B>> RedEx<(A, B)> for (X, Y) {
    fn reduce_dbi(self, subst: &Subst) -> (A, B) {
        let (x, y) = self;
        (x.reduce_dbi(subst), y.reduce_dbi(subst))
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
