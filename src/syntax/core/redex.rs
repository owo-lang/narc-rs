use super::{Closure, Elim, Term, Val};
use voile_util::uid::{DBI, GI};

/// Reducible expressions.
pub trait RedEx<T: Sized = Term>: Sized {
    /// This is primarily a private implementation-related API.
    /// Use at your own risk.
    fn reduce_dbi(self, arg: Val, dbi: DBI) -> T;

    /// When the argument is not likely to be used,
    /// prefer this over [`reduce_dbi`](reduce_dbi).
    fn reduce_dbi_borrow(self, arg: &Val, dbi: DBI) -> T;
}

impl RedEx for Term {
    fn reduce_dbi(self, arg: Val, dbi: DBI) -> Term {
        match self {
            Term::Whnf(n) => n.reduce_dbi(arg, dbi),
            Term::Redex(f, args) => {
                let args = args.into_iter().map(|t| t.reduce_dbi_borrow(&arg, dbi));
                Term::Redex(f, args.collect())
            }
        }
    }

    fn reduce_dbi_borrow(self, arg: &Val, dbi: DBI) -> Term {
        match self {
            Term::Whnf(n) => n.reduce_dbi_borrow(&arg, dbi),
            Term::Redex(f, args) => {
                let args = args.into_iter().map(|t| t.reduce_dbi_borrow(arg, dbi));
                Term::Redex(f, args.collect())
            }
        }
    }
}

impl RedEx<Elim> for Elim {
    fn reduce_dbi(self, arg: Val, dbi: DBI) -> Elim {
        match self {
            Elim::App(term) => Elim::app(term.reduce_dbi(arg, dbi)),
            e => e,
        }
    }

    fn reduce_dbi_borrow(self, arg: &Val, dbi: DBI) -> Elim {
        match self {
            Elim::App(term) => Elim::app(term.reduce_dbi_borrow(arg, dbi)),
            e => e,
        }
    }
}

impl RedEx for Val {
    fn reduce_dbi(self, arg: Val, dbi: DBI) -> Term {
        let reduce_vec = |a: Vec<Term>| {
            a.into_iter()
                .map(|a| a.reduce_dbi_borrow(&arg, dbi))
                .collect()
        };
        match self {
            Val::Pi(plicit, param_type, closure) => Term::pi(
                plicit,
                param_type.reduce_dbi_borrow(&arg, dbi),
                closure.reduce_dbi(arg, dbi + 1),
            ),
            Val::Cons(name, a) => Term::cons(name, reduce_vec(a)),
            Val::Type(n) => Term::universe(n),
            Val::Data(kind, gi, a) => Term::data(kind, gi, reduce_vec(a)),
            Val::Meta(m, a) => unimplemented!(),
            Val::App(f, args) => unimplemented!(),
            Val::Axiom(a) => Term::Whnf(Val::Axiom(a)),
            Val::Refl => Term::reflexivity(),
            Val::Id(ty, a, b) => Term::identity(
                ty.reduce_dbi_borrow(&arg, dbi),
                a.reduce_dbi_borrow(&arg, dbi),
                b.reduce_dbi(arg, dbi),
            ),
        }
    }

    fn reduce_dbi_borrow(self, arg: &Val, dbi: DBI) -> Term {
        let reduce_vec = |a: Vec<Term>| {
            a.into_iter()
                .map(|a| a.reduce_dbi_borrow(&arg, dbi))
                .collect()
        };
        match self {
            Val::Pi(plicit, param_type, closure) => Term::pi(
                plicit,
                param_type.reduce_dbi_borrow(arg, dbi),
                closure.reduce_dbi_borrow(arg, dbi + 1),
            ),
            Val::Cons(name, a) => Term::cons(name, reduce_vec(a)),
            Val::Type(n) => Term::universe(n),
            Val::Data(kind, gi, a) => Term::data(kind, gi, reduce_vec(a)),
            Val::Meta(m, a) => unimplemented!(),
            Val::App(f, args) => unimplemented!(),
            Val::Axiom(a) => Term::Whnf(Val::Axiom(a)),
            Val::Refl => Term::reflexivity(),
            Val::Id(ty, a, b) => Term::identity(
                ty.reduce_dbi_borrow(arg, dbi),
                a.reduce_dbi_borrow(arg, dbi),
                b.reduce_dbi_borrow(arg, dbi),
            ),
        }
    }
}

impl RedEx<Closure> for Closure {
    fn reduce_dbi(self, arg: Val, dbi: DBI) -> Self {
        use Closure::*;
        let Plain(body) = self;
        Self::plain(body.reduce_dbi(arg, dbi))
    }

    fn reduce_dbi_borrow(self, arg: &Val, dbi: DBI) -> Self {
        use Closure::*;
        let Plain(body) = self;
        Self::plain(body.reduce_dbi_borrow(arg, dbi))
    }
}
