use crate::syntax::core::{TermInfo, Val};

pub use self::error::*;
pub use self::state::*;

/// `Control.Monad.Except`, as type-checking error.
mod error;
/// `Control.Monad.State`, as type-checking state.
mod state;

/// Type-Checking Monad.
pub type TCM<T = TCS> = Result<T, TCE>;

/// Type-Checking Monad with State.
pub type TCMS<T> = TCM<(T, TCS)>;

/// Term-Producing Type-Checking Monad.
pub type TermTCM = TCMS<TermInfo>;

/// Whnf-Producing Type-Checking Monad.
pub type ValTCM = TCMS<Val>;
