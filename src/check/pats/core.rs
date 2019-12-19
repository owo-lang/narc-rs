use std::convert::TryFrom;

use voile_util::uid::DBI;

use crate::syntax::{
    core::{subst::DeBruijn, Elim, Term},
    pat::{Copat, Pat},
};

pub type CoreCopat = Copat<DBI, Term>;
pub type CorePat = Pat<DBI, Term>;

impl TryFrom<CoreCopat> for Term {
    type Error = String;
    fn try_from(p: CoreCopat) -> Result<Term, String> {
        Elim::from(p).try_into_app()
    }
}

impl TryFrom<CorePat> for Term {
    type Error = String;
    fn try_from(p: CorePat) -> Result<Term, String> {
        Term::try_from(Copat::App(p))
    }
}

impl From<CoreCopat> for Elim {
    fn from(p: CoreCopat) -> Elim {
        match p {
            Copat::Proj(field) => Elim::Proj(field),
            Copat::App(p) => From::from(p),
        }
    }
}

impl From<CorePat> for Elim {
    fn from(p: CorePat) -> Elim {
        match p {
            Pat::Var(ix) => Elim::from_dbi(ix),
            Pat::Forced(t) => Elim::app(t),
            Pat::Refl => Elim::app(Term::reflexivity()),
            Pat::Cons(_, head, args) => Elim::app(Term::cons(
                head,
                args.into_iter()
                    .map(Into::into)
                    .map(Elim::into_app)
                    .collect(),
            )),
            // what?
            Pat::Absurd => unreachable!(),
        }
    }
}
