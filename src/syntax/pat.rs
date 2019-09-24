use crate::syntax::core::ConHead;

/// Patterns.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pat<Ix, Term> {
    /// Variable pattern.
    /// TODO: what should be in this variant?
    ///  As we're using DBI, no name need to be stored;
    ///  while the $PV(\bar{p})$ function seems to depend on variable names.
    /// Note: it has a name suggestion in Agda.
    /// https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Internal.html#t:Pattern
    Var(Ix),
    /// Dual to [Refl](../ast/enum.Val.html#variant.Refl).
    Refl,
    /// Impossible pattern.
    Absurd,
    /// Dual to [Cons](../ast/enum.Val.html#variant.Cons),
    /// but can be forced (the first member is "is\_forced").
    Cons(bool, ConHead, Vec<Self>),
    /// Forced term as an expression.
    Forced(Term),
}

/// Copatterns.
/// The `Ix` is the representation of variable abstraction,
/// like `UID` in abstract or `DBI` in core.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Copat<Ix, Term> {
    /// Application copatterns.
    App(Pat<Ix, Term>),
    /// Projection copatterns.
    Proj(String),
}
