use crate::check::monad::{TCM, TCS};
use crate::syntax::abs::{AbsClause, AbsCopat};
use crate::syntax::core::{Clause, Pat, Tele, Term, Val};

/// A user pattern and a core term that they should equal
/// after splitting is complete.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.html#ProblemEq).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProblemEq {
    /// The pattern causes this problem.
    in_pat: AbsCopat,
    inst: Term,
    ty: Term,
}

/// User patterns we still have to split on.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.Problem.html#Problem).
#[derive(Debug, Clone)]
pub struct Problem {
    /// User patterns.
    eqs: Vec<ProblemEq>,
    /// List of user patterns which could not yet be typed.
    rest_pats: Vec<AbsCopat>,
}

/// State worked on during lhs checking.
#[derive(Debug, Clone)]
pub struct LhsState {
    /// Pattern variables' types.
    tele: Tele,
    /// Patterns after splitting. Indices are positioned from right to left.
    pats: Vec<Pat>,
    /// User patterns' unification problems.
    problem: Problem,
    /// Type eliminated by `problem.rest_pats`.
    target: Term,
    // TODO: what is `_lhsPartialSplit`?
}

pub fn init_lhs_state(tele: Tele) -> TCM<LhsState> {
    unimplemented!()
}

/// Checking an abstract clause.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.Def.html#checkClause).
pub fn clause(tcs: TCS, cls: AbsClause, against: &Val) -> TCM<Clause> {
    // Expand pattern synonyms here once we support it.
    unimplemented!()
}
