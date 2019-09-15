use crate::syntax::core::{Decl, Tele, Term};
use voile_util::meta::MetaContext;

/// Typing context.
pub type Sigma = Vec<Decl>;

/// Type-checking state.
#[derive(Debug, Clone, Default)]
pub struct TCS {
    /// Global typing context.
    pub sigma: Sigma,
    /// Local typing context.
    pub gamma: Tele,
    /// Meta variable context. Always global.
    pub meta_context: MetaContext<Term>,
}
