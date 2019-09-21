use voile_util::uid::DBI;

use super::{Pat, Term};

pub use self::apply::*;
pub use self::redex::*;

/// Substitution type.
pub struct PrimSubst<T> {
    /// The things to be substituted.
    elims: Vec<T>,
}

impl<T> PrimSubst<T> {
    pub fn lookup(&self, dbi: DBI) -> &T {
        unimplemented!()
    }
}

pub type Subst = PrimSubst<Term>;
pub type PatSubst = PrimSubst<Pat>;

/// Eliminate something with something else.
mod apply;
/// Reduction function (red-ex stands for **red**ducible **ex**pression).
mod redex;
