use crate::syntax::core::{TermInfo, Val};

pub use self::error::*;
pub use self::state::*;

/// `Control.Monad.Except`, as type-checking error.
mod error;
/// `Control.Monad.State`, as type-checking state.
mod state;

/// Type-Checking Monad.
pub type TCM<T = TCS> = Result<T, TCE>;

/// Val-Producing Type-Checking Monad.
pub type TermTCM = TCM<(TermInfo, TCS)>;

/// Whnf-Producing Type-Checking Monad.
pub type ValTCM = TCM<(Val, TCS)>;
