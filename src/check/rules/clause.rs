use voile_util::uid::DBI;

use crate::check::monad::{TCM, TCMS, TCS};
use crate::syntax::abs::{AbsClause, AbsCopat};
use crate::syntax::core::subst::RedEx;
use crate::syntax::core::{Clause, Pat, Tele, Term};
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

#[derive(Debug, Clone)]
pub struct Problem {
    /// List of user patterns which could not yet be typed.
    pub todo_pats: Vec<AbsCopat>,
    /// User patterns' unification problems.
    pub equations: Vec<Equation>,
}

/// State worked on during lhs checking.
#[derive(Debug, Clone)]
pub struct LhsState {
    /// Pattern variables' types.
    pub tele: Tele,
    /// Patterns after splitting.
    /// Indices are positioned from right to left.
    pub pats: Vec<Pat>,
    /// Yet solved pattern matching.
    pub problem: Problem,
    /// Type eliminated by `problem`.
    pub target: Term,
}

/// In Agda,
/// [this function](https://hackage.haskell.org/package/Agda-2.5.4/docs/src/Agda.TypeChecking.Rules.LHS.ProblemRest.html#initLHSState)
/// is implemented via an
/// [auxiliary function](https://hackage.haskell.org/package/Agda-2.5.4/docs/src/Agda.TypeChecking.Rules.LHS.ProblemRest.html#updateProblemRest).
pub fn init_lhs_state(pats: Vec<AbsCopat>, ty: Term) -> TCM<LhsState> {
    let (tele, target) = ty.tele_view();
    let pats_len = pats.len();
    let mut pats_iter = pats.into_iter();
    let mut equations = Vec::with_capacity(pats_len);
    for (i, bind) in tele.iter().enumerate() {
        let mut f = |pat: AbsCopat| {
            let equation = Equation {
                in_pat: pat,
                // DBI is from right to left
                inst: Term::from_dbi(DBI(pats_len - i - 1)),
                ty: bind.ty.clone(),
            };
            equations.push(equation);
        };
        if bind.is_implicit() {
            f(AbsCopat::fresh_var());
        } else if let Some(pat) = pats_iter.next() {
            f(pat);
        } else {
            // All patterns are eliminated -- because
            // `pats_iter.next()` returns `None`
            break;
        }
    }
    let problem = Problem {
        todo_pats: pats_iter.collect(),
        equations,
    };
    let tele_dbi = (0..tele.len()).rev().map(DBI).map(Pat::var).collect();
    let state = LhsState {
        tele,
        pats: tele_dbi,
        problem,
        target,
    };
    Ok(state)
}

/// Checking an abstract clause.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.Def.html#checkClause).
pub fn clause(tcs: TCS, cls: AbsClause, against: Term) -> TCMS<Clause> {
    // Expand pattern synonyms here once we support it.
    let lhs_state = init_lhs_state(cls.patterns, against)?;
    let splits_to_try = (lhs_state.problem.equations.iter())
        .filter(|e| e.in_pat.is_split())
        .cloned()
        .collect::<Vec<_>>();
    unimplemented!()
}
