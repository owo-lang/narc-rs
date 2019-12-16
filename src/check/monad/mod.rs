use crate::syntax::abs::desugar::DesugarState;
use crate::syntax::core::{Tele, TermInfo, Val};

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

impl TCS {
    /// The `tele` won't be affected after this function is invoked.
    /// This is equivalence to the type instance of `AddContext Telescope` in Agda.
    pub fn under<T>(self, tele: &mut Tele, f: impl FnOnce(TCS) -> TCMS<T>) -> TCMS<T> {
        let mut tcs = self;
        let gamma_init_len = tcs.gamma.len();
        tcs.gamma.append(tele);
        let (res, mut tcs) = f(tcs)?;
        let mut tele_recover = tcs.gamma.split_off(gamma_init_len);
        tele.append(&mut tele_recover);
        Ok((res, tcs))
    }

    pub fn considerate_of(desugar: &DesugarState) -> Self {
        let mut tcs = TCS::default();
        tcs.meta_context.expand_with_fresh_meta(desugar.meta_count);
        tcs.reserve_local_variables(desugar.decls.len());
        tcs
    }
}

/// Term-Producing Type-Checking Monad.
pub type TermTCM = TCMS<TermInfo>;

/// Whnf-Producing Type-Checking Monad.
pub type ValTCM = TCMS<Val>;
