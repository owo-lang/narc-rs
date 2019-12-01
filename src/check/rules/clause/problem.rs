use std::collections::HashMap;
use std::ops::Add;

use voile_util::uid::{DBI, UID};

use crate::check::monad::TCM;
use crate::syntax::abs::AbsCopat;
use crate::syntax::core::subst::DeBruijn;
use crate::syntax::core::{Pat, Tele, Term};
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

impl LhsState {
    /// Number of patterns.
    pub fn len_pats(&self) -> usize {
        self.pats.iter().take_while(|pat| !pat.is_proj()).count()
    }
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

/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.Problem.html#AsBinding).
#[derive(Debug, Clone)]
pub struct AsBind {
    pub name: UID,
    pub term: Term,
    pub ty: Term,
}

/// Classified patterns, called `LeftoverPatterns` in Agda.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.html#LeftoverPatterns).
#[derive(Debug, Clone)]
pub struct PatClass {
    /// Number of absurd patterns.
    pub absurd_count: usize,
    pub as_binds: Vec<AsBind>,
    pub other_pats: Vec<AbsCopat>,
    /// Supposed to be an `IntMap`.
    pub pat_vars: HashMap<DBI, UID>,
}

impl Add for PatClass {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.other_pats.append(&mut rhs.other_pats);
        self.as_binds.append(&mut rhs.as_binds);
        self.pat_vars.extend(rhs.pat_vars.into_iter());
        Self {
            absurd_count: self.absurd_count + rhs.absurd_count,
            ..self
        }
    }
}
