use std::convert::TryFrom;

use voile_util::uid::DBI;

use crate::syntax::core::subst::DeBruijn;
use crate::syntax::core::Elim;
use crate::syntax::pat;

use super::Term;

pub type Pat = pat::Copat<DBI, Term>;
pub type APat = pat::Pat<DBI, Term>;

impl TryFrom<Pat> for Term {
    type Error = String;
    fn try_from(p: Pat) -> Result<Term, String> {
        Elim::from(p).try_into_app()
    }
}

impl TryFrom<APat> for Term {
    type Error = String;
    fn try_from(p: APat) -> Result<Term, String> {
        Term::try_from(pat::Copat::App(p))
    }
}

impl From<Pat> for Elim {
    fn from(p: Pat) -> Elim {
        use pat::Copat::*;
        match p {
            Proj(field) => Elim::Proj(field),
            App(p) => From::from(p),
        }
    }
}

impl From<APat> for Elim {
    fn from(p: APat) -> Elim {
        use pat::Pat::*;
        match p {
            Var(ix) => Elim::from_dbi(ix),
            Forced(t) => Elim::app(t),
            Refl => Elim::app(Term::reflexivity()),
            Cons(_, head, args) => Elim::app(Term::cons(
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
