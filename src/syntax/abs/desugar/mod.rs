use crate::syntax::surf::ExprDecl;

use self::decls::*;
use self::error::*;
use self::exprs::*;
use self::monad::*;

/// Desugar declarations.
mod decls;
/// Desugar error.
mod error;
/// Desugar expressions.
mod exprs;
/// Desugar monad (state and monad-result).
mod monad;

pub fn desugar_main(decls: Vec<ExprDecl>) -> DesugarM {
    desugar_decls(DesugarState::with_capacity(decls.len()), decls)
}

#[cfg(test)]
mod tests;
