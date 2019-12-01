use crate::check::monad::{TCMS, TCS};
use crate::syntax::abs::AbsClause;
use crate::syntax::core::{Clause, Term};

pub use self::lhs::*;
pub use self::problem::*;

mod lhs;
mod problem;

pub fn check_rhs(tcs: TCS, lhs: LhsState) -> TCMS<Clause> {
    debug_assert!(lhs.problem.todo_pats.is_empty());
    let len_pats = lhs.len_pats();
    unimplemented!()
}

/// Checking an abstract clause.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.Def.html#checkClause).
pub fn clause(tcs: TCS, cls: AbsClause, against: Term) -> TCMS<Clause> {
    // Expand pattern synonyms here once we support it.
    let lhs_state = init_lhs_state(cls.patterns, against)?;
    check_lhs(tcs, lhs_state)
}
