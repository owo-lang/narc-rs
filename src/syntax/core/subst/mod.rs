pub use self::{apply::*, dbi::*, prim::*, redex::*};

pub type Subst = PrimSubst<crate::syntax::core::Term>;

/// Eliminate something with something else.
mod apply;
/// De-Bruijn indices things.
mod dbi;
/// The primitive substitution type.
mod prim;
/// Reduction function (red-ex stands for **red**ducible **ex**pression).
mod redex;
