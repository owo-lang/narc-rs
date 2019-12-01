use crate::syntax::abs::AbsCopat;
use crate::syntax::core::Term;
use crate::syntax::pat::PatCommon;

/// A user pattern and a core term that they should equal
/// after splitting is complete.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.html#ProblemEq).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Equation {
    /// The pattern causes this problem.
    pub in_pat: AbsCopat,
    pub inst: Term,
    pub ty: Term,
}

impl PatCommon for Equation {
    fn is_split(&self) -> bool {
        self.in_pat.is_split()
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    /// List of user patterns which could not yet be typed.
    pub todo_pats: Vec<AbsCopat>,
    /// User patterns' unification problems.
    pub equations: Vec<Equation>,
}

impl Problem {
    pub fn is_all_solved(&self) -> bool {
        self.todo_pats.is_empty() && self.equations.iter().all(|eq| eq.is_solved())
    }
}
