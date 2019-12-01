use std::convert::TryFrom;

use voile_util::uid::DBI;

use crate::check::monad::{TCM, TCMS, TCS};
use crate::syntax::abs::AbsCopat;
use crate::syntax::core::subst::{RedEx, Subst};
use crate::syntax::core::{Clause, Pat, Tele, Term};
use crate::syntax::pat::PatCommon;

use super::super::ERROR_TAKE;
use super::{Equation, Problem};

/**
Compute substitution from the out patterns.

We have two slightly different cases here: normal function and
with-function. In both cases the goal is to build a substitution
from the context $\Gamma$ of the previous checkpoint to the current lhs
context $\Delta$:
$$
  \Delta \vdash \text{paramSub} : \Gamma
$$

## Normal function, `f`

$$
\begin{aligned}
  \Gamma & = \text{cxt = module parameter telescope of f} \\\\
  \Psi &= \text{non-parameter arguments of } f
    (\text{we have} f : \Gamma~\Psi \rarr A) \\\\
  \Delta      & \vdash \text{patSub}  : \Gamma~\Psi \\\\
  \Gamma~\Psi & \vdash \text{weakSub} : \Gamma \\\\
  \text{paramSub} &= \text{patSub} \circ \text{weakSub}
\end{aligned}
$$

# With-function

Not supported, but comments are preserved for future references.

$$
\begin{aligned}
  \Gamma &= \text{lhs context of the parent clause } (cxt = []) \\\\
  \Psi &= \text{argument telescope of with-function} \\\\
  \Theta &= \text{inserted implicit patterns not in } \Psi (\text{agda issue 2827}) \\\\
     & \text{(this happens if the goal computes to an implicit} \\\\
     & \text{function type after some matching in the with-clause)} \\\\
  \Psi        &\vdash \text{withSub} : \Gamma \\\\
  \Delta      &\vdash \text{patSub}  : \Psi~\Theta \\\\
  \Psi~\Theta &\vdash \text{weakSub} : \Psi \\\\
  \text{paramSub} &= \text{patSub} \circ \text{weakSub} \circ \text{withSub}
\end{aligned}
$$

To compute $\Theta$ we can look at the arity of the with-function
and compare it to numPats. This works since the with-function
type is fully reduced.
*/
pub fn final_check(tcs: TCS, lhs: LhsState) -> TCMS<Clause> {
    debug_assert!(lhs.problem.todo_pats.is_empty());
    let len_pats = lhs.len_pats();
    // It should be `len_pats - ctx.len()`,
    // but I think the `ctx` in Agda comes from module parameters | variables | stuff,
    // which we don't really support.
    let weak_sub = Subst::weaken(Default::default(), len_pats);
    let with_sub = Default::default();
    let pat_sub = Subst::concat(
        (lhs.pats.iter().take(len_pats).rev().cloned())
            .map(Term::try_from)
            .map(|t| t.expect(ERROR_TAKE)),
        Default::default(),
    );
    let param_sub = Subst::compose(Subst::compose(pat_sub, weak_sub), with_sub);
    unimplemented!()
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

/// Checking a pattern matching lhs recursively.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.html).
pub fn check_lhs(tcs: TCS, lhs: LhsState) -> TCMS<Clause> {
    if lhs.problem.is_all_solved() {
        return final_check(tcs, lhs);
    }
    let splits_to_try = (lhs.problem.equations.iter())
        .filter(|e| e.in_pat.is_split())
        .cloned()
        .collect::<Vec<_>>();
    unimplemented!()
}
