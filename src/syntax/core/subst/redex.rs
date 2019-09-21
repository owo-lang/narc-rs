use super::super::{Closure, Elim, Term, Val};
use super::{def_app, Substitution};

/// Reducible expressions.
pub trait RedEx<T: Sized = Term>: Sized {
    /// This is primarily a private implementation-related API.
    /// Use at your own risk.
    fn reduce_dbi(self, subst: Substitution) -> T;

    /// When the argument is not likely to be used,
    /// prefer this over [`reduce_dbi`](reduce_dbi).
    fn reduce_dbi_borrow(self, subst: &Substitution) -> T;
}

impl RedEx for Term {
    fn reduce_dbi(self, subst: Substitution) -> Term {
        match self {
            Term::Whnf(n) => n.reduce_dbi(subst),
            Term::Redex(f, args) => def_app(f, vec![], reduce_vec_dbi(args, &subst)),
        }
    }

    fn reduce_dbi_borrow(self, subst: &Substitution) -> Term {
        match self {
            Term::Whnf(n) => n.reduce_dbi_borrow(&subst),
            Term::Redex(f, args) => def_app(f, vec![], reduce_vec_dbi(args, &subst)),
        }
    }
}

impl RedEx<Elim> for Elim {
    fn reduce_dbi(self, subst: Substitution) -> Elim {
        match self {
            Elim::App(term) => Elim::app(term.reduce_dbi(subst)),
            e => e,
        }
    }

    fn reduce_dbi_borrow(self, subst: &Substitution) -> Elim {
        match self {
            Elim::App(term) => Elim::app(term.reduce_dbi_borrow(subst)),
            e => e,
        }
    }
}

impl RedEx for Val {
    fn reduce_dbi(self, subst: Substitution) -> Term {
        match self {
            Val::Pi(plicit, param_type, closure) => Term::pi(
                plicit,
                param_type.reduce_dbi_borrow(&subst),
                closure.reduce_dbi(subst),
            ),
            Val::Cons(name, a) => Term::cons(name, reduce_vec_dbi(a, &subst)),
            Val::Type(n) => Term::universe(n),
            Val::Data(kind, gi, a) => Term::data(kind, gi, reduce_vec_dbi(a, &subst)),
            Val::Meta(m, a) => Term::meta(m, reduce_vec_dbi(a, &subst)),
            Val::App(f, args) => unimplemented!(),
            Val::Axiom(a) => Term::Whnf(Val::Axiom(a)),
            Val::Refl => Term::reflexivity(),
            Val::Id(ty, a, b) => Term::identity(
                ty.reduce_dbi_borrow(&subst),
                a.reduce_dbi_borrow(&subst),
                b.reduce_dbi(subst),
            ),
        }
    }

    fn reduce_dbi_borrow(self, subst: &Substitution) -> Term {
        let reduce_vec =
            |a: Vec<Term>| a.into_iter().map(|a| a.reduce_dbi_borrow(&subst)).collect();
        match self {
            Val::Pi(plicit, param_type, closure) => Term::pi(
                plicit,
                param_type.reduce_dbi_borrow(subst),
                closure.reduce_dbi_borrow(subst),
            ),
            Val::Cons(name, a) => Term::cons(name, reduce_vec(a)),
            Val::Type(n) => Term::universe(n),
            Val::Data(kind, gi, a) => Term::data(kind, gi, reduce_vec(a)),
            Val::Meta(m, a) => Term::meta(m, reduce_vec_dbi(a, &subst)),
            Val::App(f, args) => unimplemented!(),
            Val::Axiom(a) => Term::Whnf(Val::Axiom(a)),
            Val::Refl => Term::reflexivity(),
            Val::Id(ty, a, b) => Term::identity(
                ty.reduce_dbi_borrow(subst),
                a.reduce_dbi_borrow(subst),
                b.reduce_dbi_borrow(subst),
            ),
        }
    }
}

impl RedEx<Closure> for Closure {
    fn reduce_dbi(self, subst: Substitution) -> Self {
        use Closure::*;
        let Plain(body) = self;
        Self::plain(body.reduce_dbi(subst))
    }

    fn reduce_dbi_borrow(self, subst: &Substitution) -> Self {
        use Closure::*;
        let Plain(body) = self;
        Self::plain(body.reduce_dbi_borrow(subst))
    }
}

fn reduce_vec_dbi<T>(me: Vec<impl RedEx<T>>, subst: &Substitution) -> Vec<T> {
    me.into_iter().map(|e| e.reduce_dbi_borrow(subst)).collect()
}
