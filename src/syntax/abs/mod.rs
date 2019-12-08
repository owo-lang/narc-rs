pub use self::ast::*;
pub use self::decl::*;
pub use self::decl_impl::*;
pub use self::pretty::*;

/// Abstract terms.
mod ast;
/// Declarations.
mod decl;
/// Declarations' trivial trait implementations.
mod decl_impl;
/// Surface to abstract, scope-checking.
pub mod desugar;
mod pretty;
