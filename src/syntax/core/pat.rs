use voile_util::uid::DBI;

use crate::syntax::core::subst::RedEx;
use crate::syntax::core::Elim;
use crate::syntax::pat;

use super::Term;
use std::convert::TryInto;

pub type Pat = pat::Copat<DBI, Term>;
pub type APat = pat::Pat<DBI, Term>;

impl TryInto<Term> for Pat {
    type Error = String;
    fn try_into(self) -> Result<Term, String> {
        Into::<Elim>::into(self).try_into_app()
    }
}

impl TryInto<Term> for APat {
    type Error = String;
    fn try_into(self) -> Result<Term, String> {
        pat::Copat::App(self).try_into()
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

impl Into<Elim> for APat {
    fn into(self) -> Elim {
        use pat::Pat::*;
        match self {
            Var(ix) => Elim::from_dbi(ix),
            Forced(t) => Elim::app(t),
            Refl => Elim::app(Term::reflexivity()),
            Cons(is_forced, head, args) => Elim::app(Term::cons(
                head,
                args.into_iter()
                    .map(Into::into)
                    .map(Elim::into_app)
                    .collect(),
            )),
            // what?
            Absurd => unimplemented!(),
        }
    }
}
