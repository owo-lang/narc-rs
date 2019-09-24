use voile_util::uid::DBI;

use crate::syntax::pat;

use super::Term;
use crate::syntax::core::Elim;

pub type Pat = pat::Copat<DBI, Term>;

impl Pat {
    pub fn into_elim(self) -> Elim {
        use pat::Copat::*;
        match self {
            App(pat) => unimplemented!(),
            Proj(field) => Elim::Proj(field),
        }
    }
}
