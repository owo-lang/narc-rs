use super::super::{Closure, Elim, Term, Val};
use super::{def_app, Subst};

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
            Term::Redex(f, args) => def_app(f, vec![], reduce_vec_dbi(args, &subst)),
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

impl RedEx for Val {
    fn reduce_dbi(self, subst: &Subst) -> Term {
        let reduce_vec = |a: Vec<Term>| a.into_iter().map(|a| a.reduce_dbi(&subst)).collect();
        match self {
            Val::Pi(plicit, param_type, closure) => Term::pi(
                plicit,
                param_type.reduce_dbi(subst),
                closure.reduce_dbi(subst),
            ),
            Val::Cons(name, a) => Term::cons(name, reduce_vec(a)),
            Val::Type(n) => Term::universe(n),
            Val::Data(kind, gi, a) => Term::data(kind, gi, reduce_vec(a)),
            Val::Meta(m, a) => Term::meta(m, reduce_vec_dbi(a, &subst)),
            Val::App(f, args) => subst
                .lookup(f)
                .map(|o| o.clone())
                .unwrap_or_else(|dbi| Term::Whnf(Val::App(dbi, vec![])))
                .apply_elim(reduce_vec_dbi(args, subst)),
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
