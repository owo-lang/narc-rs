use voile_util::uid::DBI;

use crate::check::monad::{TCM, TCMS, TCS};
use crate::syntax::abs::{AbsClause, AbsCopat};
use crate::syntax::core::{Clause, Pat, Tele, Term};

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
    /// Type eliminated by `problem`.
    pub target: Term,
    // TODO: what is `_lhsPartialSplit`?
}

/// In Agda,
/// [this function](https://hackage.haskell.org/package/Agda-2.5.4/docs/src/Agda.TypeChecking.Rules.LHS.ProblemRest.html#initLHSState)
/// is implemented via an
/// [auxiliary function](https://hackage.haskell.org/package/Agda-2.5.4/docs/src/Agda.TypeChecking.Rules.LHS.ProblemRest.html#updateProblemRest).
pub fn init_lhs_state(pats: Vec<AbsCopat>, ty: Term) -> TCM<LhsState> {
    let (tele, target) = ty.tele_view();
    let pats_len = pats.len();
    let mut pats_iter = pats.into_iter();
    let mut pats = Vec::with_capacity(pats_len + 2);
    for bind in &tele {
        if bind.is_implicit() {
            pats.push(AbsCopat::fresh_var());
        } else if let Some(pat) = pats_iter.next() {
            pats.push(pat);
        } else {
            // All patterns are eliminated -- because
            // `pats_iter.next()` returns `None`
            break;
        }
    }
    let tele_dbi = (0..tele.len()).rev().map(DBI).map(Pat::var).collect();
    let state = LhsState {
        tele,
        pats: tele_dbi,
        problem: pats,
        target,
    };
    Ok(state)
}

/// Checking an abstract clause.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.Def.html#checkClause).
pub fn clause(tcs: TCS, cls: AbsClause, against: Term) -> TCMS<Clause> {
    // Expand pattern synonyms here once we support it.
    unimplemented!()
}
