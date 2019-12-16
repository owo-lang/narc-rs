use crate::syntax::surf::ExprDecl;

pub use self::monad::*;
use self::{decls::*, error::*, exprs::*};

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
