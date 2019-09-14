use super::Term;

/// Patterns.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pat<Term> {
    /// Variable pattern.
    /// TODO: what should be in this variant?
    ///  As we're using DBI, no name need to be stored;
    ///  while the $PV(\bar{p})$ function seems to depend on variable names.
    /// Note: it has a name suggestion in Agda.
    /// https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Internal.html#t:Pattern
    Var,
    /// Dual to [Refl](../ast/enum.Val.html#variant.Refl).
    Refl,
    /// Impossible pattern.
    Absurd,
    /// Dual to [Cons](../ast/enum.Val.html#variant.Cons),
    /// but can be forced (the first member is "is\_forced").
    Cons(bool, Vec<Self>),
    /// Forced term as an expression.
    Forced(Term),
}

/// Copatterns.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Copat<Term> {
    /// Application copatterns.
    App(Pat<Term>),
    /// Projection copatterns.
    Proj(String),
}
