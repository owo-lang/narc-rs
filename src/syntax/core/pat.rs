use voile_util::uid::DBI;

use crate::syntax::core::subst::RedEx;
use crate::syntax::core::Elim;
use crate::syntax::pat;

use super::Term;

pub type Pat = pat::Copat<DBI, Term>;

impl Into<Term> for Pat {
    fn into(self) -> Term {
        match self.into() {
            Elim::App(t) => *t,
            _ => unreachable!(),
        }
    }
}

impl Into<Term> for pat::Pat<DBI, Term> {
    fn into(self) -> Term {
        match self.into() {
            Elim::App(t) => *t,
            _ => unreachable!(),
        }
    }
}

impl Into<Elim> for Pat {
    fn into(self) -> Elim {
        use pat::Copat::*;
        match self {
            Proj(field) => Elim::Proj(field),
            App(p) => p.into(),
        }
    }
}

impl Into<Elim> for pat::Pat<DBI, Term> {
    fn into(self) -> Elim {
        use pat::Pat::*;
        match self {
            Var(ix) => Elim::from_dbi(ix),
            Forced(t) => Elim::app(t),
            Refl => Elim::app(Term::reflexivity()),
            Cons(is_forced, head, args) => {
                Elim::app(Term::cons(head, args.into_iter().map(Into::into).collect()))
            }
            // what?
            Absurd => unimplemented!(),
        }
    }
}
