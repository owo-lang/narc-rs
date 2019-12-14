use voile_util::uid::{next_uid, UID};

use super::common::ConHead;

/// Patterns.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pat<Ix, Term> {
    /// Variable pattern.
    /// Note: it has a name suggestion in Agda.
    /// https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Internal.html#t:Pattern
    Var(Ix),
    /// Dual to [`crate::syntax::core::ast::Val::Refl`].
    Refl,
    /// Impossible pattern.
    Absurd,
    /// Dual to [`crate::syntax::core::ast::Val::Cons`],
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

/// Common methods shared by patterns.
pub trait PatCommon {
    /// Whether the pattern is a splitting pattern or not.
    fn is_split(&self) -> bool;

    fn is_solved(&self) -> bool {
        !self.is_split()
    }
}

impl<Ix, Term> PatCommon for Pat<Ix, Term> {
    fn is_split(&self) -> bool {
        use Pat::*;
        match self {
            Refl | Cons(..) => true,
            Var(..) | Absurd | Forced(..) => false,
        }
    }
}

impl<Ix, Term> PatCommon for Copat<Ix, Term> {
    fn is_split(&self) -> bool {
        match self {
            Copat::App(p) => p.is_split(),
            // Agda panics for this case.
            Copat::Proj(..) => false,
        }
    }
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

    pub fn is_proj(&self) -> bool {
        match self {
            Copat::App(_) => false,
            Copat::Proj(_) => true,
        }
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

impl<Term> Copat<UID, Term> {
    pub fn fresh_var() -> Self {
        Self::var(unsafe { next_uid() })
    }
}
