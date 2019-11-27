use crate::check::monad::{TCM, TCS};
use crate::syntax::abs::{AbsClause, AbsCopat};
use crate::syntax::core::{Clause, Pat, Tele, Term, Val};

/// A user pattern and a core term that they should equal
/// after splitting is complete.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.html#ProblemEq).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProblemEq {
    /// The pattern causes this problem.
    pub in_pat: AbsCopat,
    pub inst: Term,
    pub ty: Term,
}

/// State worked on during lhs checking.
#[derive(Debug, Clone)]
pub struct LhsState {
    /// Pattern variables' types.
    pub tele: Tele,
    /// Patterns after splitting. Indices are positioned from right to left.
    pub pats: Vec<Pat>,
    /// User patterns' unification problems.
    pub problem: Vec<AbsCopat>,
    /// Type eliminated by `problem.rest_pats`.
    pub target: Term,
    // TODO: what is `_lhsPartialSplit`?
}

pub fn init_lhs_state(tcs: TCS, tele: Tele, ty: &Val) -> TCM<LhsState> {
    unimplemented!()
}

/// Checking an abstract clause.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.Def.html#checkClause).
pub fn clause(tcs: TCS, cls: AbsClause, against: &Val) -> TCM<Clause> {
    // Expand pattern synonyms here once we support it.
    unimplemented!()
}
