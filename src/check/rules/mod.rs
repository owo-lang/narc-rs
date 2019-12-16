pub use self::{decls::*, term::simplify};

pub const ERROR_MSG: &str = "Please report this as a bug.";

/// Type check a function clause.
mod clause;
/// Type check data type & constructor declarations.
mod data;
/// Check a list of declarations.
mod decls;
/// Type check a term.
mod term;
