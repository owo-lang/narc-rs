use crate::syntax::surf::ExprDecl;

use self::decls::*;
use self::error::*;
use self::monad::*;

/// Desugar declarations.
mod decls;
/// Desugar error.
mod error;
/// Desugar monad (state and monad-result).
mod monad;

pub fn desugar_main(decls: Vec<ExprDecl>) -> Result<DesugarState, DesugarErr> {
    desugar_decls(DesugarState::with_capacity(decls.len()), decls)
}
