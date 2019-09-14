use voile_util::uid::DBI;

use super::{Closure, Term, Val};

/// Reducible expressions.
pub trait RedEx<T: Sized = Term>: Sized {
    /// This is primarily a private implementation-related API.
    /// Use at your own risk.
    fn reduce_with_dbi(self, arg: Val, dbi: DBI) -> T;

    /// When the argument is not likely to be used,
    /// prefer this over [`reduce_with_dbi`](reduce_with_dbi).
    fn reduce_with_dbi_borrow(self, arg: &Val, dbi: DBI) -> T;
}

impl RedEx for Term {
    fn reduce_with_dbi(self, arg: Val, dbi: DBI) -> Term {
        match self {
            Term::Whnf(n) => n.reduce_with_dbi(arg, dbi),
            Term::Redex(f, args) => unimplemented!(),
        }
    }

    fn reduce_with_dbi_borrow(self, arg: &Val, dbi: DBI) -> Term {
        match self {
            Term::Whnf(n) => n.reduce_with_dbi_borrow(&arg, dbi),
            Term::Redex(f, args) => unimplemented!(),
        }
    }
}

impl RedEx for Val {
    fn reduce_with_dbi(self, arg: Val, dbi: DBI) -> Term {
        let reduce_vec = |a: Vec<Term>| {
            a.into_iter()
                .map(|a| a.reduce_with_dbi_borrow(&arg, dbi))
                .collect()
        };
        match self {
            Val::Pi(plicit, param_type, closure) => Term::pi(
                plicit,
                param_type.reduce_with_dbi_borrow(&arg, dbi),
                closure.reduce_with_dbi(arg, dbi + 1),
            ),
            Val::Cons(name, a) => Term::cons(name, reduce_vec(a)),
            Val::Type(n) => Term::universe(n),
            Val::Data(kind, a) => Term::data(kind, reduce_vec(a)),
            Val::Meta(m, a) => unimplemented!(),
            Val::App(f, args) => unimplemented!(),
            Val::Axiom(a) => Term::Whnf(Val::Axiom(a)),
            Val::Refl => Term::reflexivity(),
            Val::Id(ty, a, b) => Term::identity(
                ty.reduce_with_dbi_borrow(&arg, dbi),
                a.reduce_with_dbi_borrow(&arg, dbi),
                b.reduce_with_dbi(arg, dbi),
            ),
        }
    }

    fn reduce_with_dbi_borrow(self, arg: &Val, dbi: DBI) -> Term {
        let reduce_vec = |a: Vec<Term>| {
            a.into_iter()
                .map(|a| a.reduce_with_dbi_borrow(&arg, dbi))
                .collect()
        };
        match self {
            Val::Pi(plicit, param_type, closure) => Term::pi(
                plicit,
                param_type.reduce_with_dbi_borrow(arg, dbi),
                closure.reduce_with_dbi_borrow(arg, dbi + 1),
            ),
            Val::Cons(name, a) => Term::cons(name, reduce_vec(a)),
            Val::Type(n) => Term::universe(n),
            Val::Data(kind, a) => Term::data(kind, reduce_vec(a)),
            Val::Meta(m, a) => unimplemented!(),
            Val::App(f, args) => unimplemented!(),
            Val::Axiom(a) => Term::Whnf(Val::Axiom(a)),
            Val::Refl => Term::reflexivity(),
            Val::Id(ty, a, b) => Term::identity(
                ty.reduce_with_dbi_borrow(arg, dbi),
                a.reduce_with_dbi_borrow(arg, dbi),
                b.reduce_with_dbi_borrow(arg, dbi),
            ),
        }
    }
}

impl RedEx<Closure> for Closure {
    fn reduce_with_dbi(self, arg: Val, dbi: DBI) -> Self {
        use Closure::*;
        let Plain(body) = self;
        Self::plain(body.reduce_with_dbi(arg, dbi))
    }

    fn reduce_with_dbi_borrow(self, arg: &Val, dbi: DBI) -> Self {
        use Closure::*;
        let Plain(body) = self;
        Self::plain(body.reduce_with_dbi_borrow(arg, dbi))
    }
}
