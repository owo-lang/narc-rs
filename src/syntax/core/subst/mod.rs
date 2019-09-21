pub use self::apply::*;
pub use self::prim::*;
pub use self::redex::*;

/// Eliminate something with something else.
mod apply;
/// The primitive substitution type.
mod prim;
/// Reduction function (red-ex stands for **red**ducible **ex**pression).
mod redex;
