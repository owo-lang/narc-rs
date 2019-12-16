pub use self::{ast::*, decl::*, decl_impl::*, pretty::*};

/// Abstract terms.
mod ast;
/// Declarations.
mod decl;
/// Declarations' trivial trait implementations.
mod decl_impl;
/// Surface to abstract, scope-checking.
pub mod desugar;
mod pretty;
