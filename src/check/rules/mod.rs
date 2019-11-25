/// Type check a function clause.
mod clause;
/// Type check data type & constructor declarations.
mod data;
/// Synthesize the type and its well-typed form from an abstract term.
mod infer;
/// Type check a term.
mod term;
/// Conversion check.
mod unify;
/// Find the weak-head-normal-form (normalize) of an expression.
/// TODO: Unfolds declarations.
mod whnf;
