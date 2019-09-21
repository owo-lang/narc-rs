use voile_util::uid::DBI;

use super::Elim;

pub use self::apply::*;
pub use self::redex::*;

/// Substitution type.
pub struct Subst {
    /// The things to be substituted.
    elims: Vec<Elim>,
}

impl Subst {
    pub fn lookup(&self, dbi: DBI) -> &Elim {
        unimplemented!()
    }
}

/// Eliminate something with something else.
mod apply;
/// Reduction function (red-ex stands for **red**ducible **ex**pression).
mod redex;
