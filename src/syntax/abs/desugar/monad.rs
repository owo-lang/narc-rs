use voile_util::meta::MI;

use crate::syntax::abs::AbsDecl;

use super::DesugarErr;

/// Desugar Monad.
pub type DesugarM<State = DesugarState> = Result<State, DesugarErr>;

#[derive(Debug, Clone, Default)]
pub struct DesugarState {
    pub decls: Vec<AbsDecl>,
    pub meta_count: MI,
}

impl DesugarState {
    pub fn with_capacity(decl_possible_size: usize) -> Self {
        Self {
            meta_count: Default::default(),
            decls: Vec::with_capacity(decl_possible_size),
        }
    }
}
