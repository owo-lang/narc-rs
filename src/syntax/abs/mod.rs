pub use self::ast::*;
pub use self::decl::*;

/// Declarations.
mod decl;

/// Abstract terms.
mod ast;

/// Declarations' trivial trait implementations.
mod decl_impl;

/// Surface to abstract, scope-checking.
pub mod desugar;
