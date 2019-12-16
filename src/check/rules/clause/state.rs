use voile_util::uid::DBI;

use crate::{
    check::{monad::TCM, pats::CoreCopat, rules::clause::Equation},
    syntax::{
        abs::AbsCopat,
        core::{subst::DeBruijn, Tele, Term},
        pat::PatCommon,
    },
};

#[derive(Debug, Clone)]
pub(super) struct Problem {
    /// List of user patterns which could not yet be typed.
    pub(super) todo_pats: Vec<AbsCopat>,
    /// User patterns' unification problems.
    pub(super) equations: Vec<Equation>,
}

impl Problem {
    pub(super) fn is_all_solved(&self) -> bool {
        self.todo_pats.is_empty() && self.equations.iter().all(|eq| eq.is_solved())
    }
}

/// State worked on during lhs checking.
#[derive(Clone)]
pub(super) struct LhsState {
    /// Pattern variables' types.
    pub(super) tele: Tele,
    /// Patterns after splitting.
    /// Indices are positioned from right to left.
    pub(super) pats: Vec<CoreCopat>,
    /// Yet solved pattern matching.
    pub(super) problem: Problem,
    /// Type eliminated by `problem`.
    pub(super) target: Term,
}

impl LhsState {
    /// Number of patterns.
    pub(super) fn len_pats(&self) -> usize {
        self.pats.iter().take_while(|pat| !pat.is_proj()).count()
    }
}

/// In Agda,
/// [this function](https://hackage.haskell.org/package/Agda-2.5.4/docs/src/Agda.TypeChecking.Rules.LHS.ProblemRest.html#initLHSState)
/// is implemented via an
/// [auxiliary function](https://hackage.haskell.org/package/Agda-2.5.4/docs/src/Agda.TypeChecking.Rules.LHS.ProblemRest.html#updateProblemRest).
pub(super) fn init_lhs_state(pats: Vec<AbsCopat>, ty: Term) -> TCM<LhsState> {
    let (tele, target) = ty.tele_view();
    let mut pats_iter = pats.into_iter();
    let tele_len = tele.len();
    let mut equations = Vec::with_capacity(tele_len);
    for (i, bind) in tele.iter().enumerate() {
        let mut f = |in_pat: AbsCopat| {
            let equation = Equation {
                in_pat,
                // DBI is from right to left
                inst: Term::from_dbi(DBI(tele_len - i - 1)),
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
    let tele_dbi = (0..tele_len).rev().map(DBI).map(CoreCopat::var).collect();
    let state = LhsState {
        tele,
        pats: tele_dbi,
        problem,
        target,
    };
    Ok(state)
}
