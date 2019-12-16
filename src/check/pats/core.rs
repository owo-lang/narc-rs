use std::convert::TryFrom;

use voile_util::uid::DBI;

use crate::syntax::{
    core::{subst::DeBruijn, Elim, Term},
    pat,
};

pub type CoreCopat = pat::Copat<DBI, Term>;
pub type CorePat = pat::Pat<DBI, Term>;

impl TryFrom<CoreCopat> for Term {
    type Error = String;
    fn try_from(p: CoreCopat) -> Result<Term, String> {
        Elim::from(p).try_into_app()
    }
}

impl TryFrom<CorePat> for Term {
    type Error = String;
    fn try_from(p: CorePat) -> Result<Term, String> {
        Term::try_from(pat::Copat::App(p))
    }
}

impl From<CoreCopat> for Elim {
    fn from(p: CoreCopat) -> Elim {
        use pat::Copat::*;
        match p {
            Proj(field) => Elim::Proj(field),
            App(p) => From::from(p),
        }
    }
}

impl From<CorePat> for Elim {
    fn from(p: CorePat) -> Elim {
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
            Absurd => unreachable!(),
        }
    }
}
