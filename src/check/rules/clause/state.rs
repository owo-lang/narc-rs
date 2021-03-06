use voile_util::uid::DBI;

use crate::{
    check::{monad::TCM, pats::CoreCopat, rules::clause::eqs::Equation},
    syntax::{
        abs::AbsCopat,
        core::{
            subst::{DeBruijn, RedEx, Subst},
            Tele, TeleS, Term,
        },
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
    pub(super) fn take_first_todo_pat(&mut self) -> Option<AbsCopat> {
        if self.todo_pats.is_empty() {
            None
        } else {
            Some(self.todo_pats.remove(0))
        }
    }

    pub(super) fn is_all_solved(&self) -> bool {
        self.todo_pats.is_empty() && self.equations.iter().all(|eq| eq.is_solved())
    }

    pub(super) fn new(todo_pats: Vec<AbsCopat>) -> Self {
        let equations = Vec::with_capacity(todo_pats.len());
        Self {
            todo_pats,
            equations,
        }
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

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.ProblemRest.html#initLHSState).
    pub(super) fn new(todo_pats: Vec<AbsCopat>, ty: Term) -> Self {
        Self {
            tele: Default::default(),
            pats: Default::default(),
            problem: Problem::new(todo_pats),
            target: ty,
        }
    }
}

/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.Implicit.html#insertImplicitPatterns).
#[allow(dead_code)]
fn insert_implicit_pats(pats: Vec<AbsCopat>, tele: &TeleS) -> Vec<AbsCopat> {
    debug_assert!(pats.len() <= tele.len());
    let mut new_pats = Vec::with_capacity(tele.len().max(pats.capacity()));
    for bind in tele {
        if bind.is_implicit() {
            new_pats.push(AbsCopat::fresh_var())
        } else {
            break;
        }
    }
    // Shouldn't insert too many patterns
    debug_assert!(new_pats.len() + pats.len() <= tele.len());
    let mut original_pats = pats;
    new_pats.append(&mut original_pats);
    new_pats
}

/// It's supposed to use [`insert_implicit_pats`](self::insert_implicit_pats) to
/// insert patterns, but as I'm refactoring it to an imperative style, reusing
/// the code becomes a bit difficult. [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.ProblemRest.html#updateProblemRest).
pub(super) fn progress_lhs_state(
    LhsState {
        pats,
        problem: Problem {
            todo_pats,
            equations,
        },
        target,
        tele: mut old_tele,
    }: LhsState,
) -> TCM<LhsState> {
    let mut pats_iter = todo_pats.into_iter();
    let (mut tele, target) = target.tele_view();
    let tele_len = tele.len();
    let mut new_equations = Vec::with_capacity(tele_len);
    for (i, bind) in tele.iter().enumerate() {
        let mut f = |in_pat: AbsCopat| {
            let equation = Equation {
                in_pat,
                // DBI is from right to left
                inst: Term::from_dbi(DBI(tele_len - i - 1)),
                ty: bind.ty.clone(),
            };
            new_equations.push(equation);
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
    let tau = Subst::raise(DBI(tele_len));
    let mut equations = equations.reduce_dbi(tau.clone());
    equations.append(&mut new_equations);
    let problem = Problem {
        todo_pats: pats_iter.collect(),
        equations,
    };
    old_tele.append(&mut tele);
    let mut pats = pats.reduce_dbi(tau);
    pats.extend((0..tele_len).rev().map(DBI).map(CoreCopat::var));
    let state = LhsState {
        tele: old_tele,
        pats,
        problem,
        target,
    };
    Ok(state)
}
