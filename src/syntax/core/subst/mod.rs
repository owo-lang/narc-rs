use super::Elim;

pub use self::apply::*;
pub use self::redex::*;

/// Substitution.
pub type Substitution = Vec<Elim>;

/// Eliminate something with something else.
mod apply;
/// Reduction function (red-ex stands for **red**ducible **ex**pression).
mod redex;
