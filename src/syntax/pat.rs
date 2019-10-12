use crate::syntax::core::ConHead;

/// Patterns.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pat<Ix, Term> {
    /// Variable pattern.
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

impl<Ix, Term> Copat<Ix, Term> {
    pub fn absurd() -> Self {
        Copat::App(Pat::Absurd)
    }
    pub fn reflexivity() -> Self {
        Copat::App(Pat::Refl)
    }
    pub fn var(ix: Ix) -> Self {
        Copat::App(Pat::Var(ix))
    }
    pub fn term(term: Term) -> Self {
        Copat::App(Pat::Forced(term))
    }
    pub fn cons(is_forced: bool, cons: ConHead, pats: Vec<Pat<Ix, Term>>) -> Self {
        Copat::App(Pat::Cons(is_forced, cons, pats))
    }

    pub fn map_app<Ix2, Term2>(
        self,
        f: impl FnOnce(Pat<Ix, Term>) -> Pat<Ix2, Term2>,
    ) -> Copat<Ix2, Term2> {
        match self {
            Copat::App(app) => Copat::App(f(app)),
            Copat::Proj(field) => Copat::Proj(field),
        }
    }
}
