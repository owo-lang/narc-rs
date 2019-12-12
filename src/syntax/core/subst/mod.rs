pub use self::apply::*;
pub use self::dbi::*;
pub use self::prim::*;
pub use self::redex::*;

pub type Subst = PrimSubst<crate::syntax::core::Term>;
pub type PatSubst = PrimSubst<crate::syntax::core::Pat>;

/// Eliminate something with something else.
mod apply;
/// De-Bruijn indices things.
mod dbi;
/// The primitive substitution type.
mod prim;
/// Reduction function (red-ex stands for **red**ducible **ex**pression).
mod redex;
