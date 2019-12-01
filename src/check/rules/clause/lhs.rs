use std::convert::TryFrom;

use voile_util::uid::DBI;

use crate::check::monad::{TCMS, TCS};
use crate::syntax::core::subst::Subst;
use crate::syntax::core::{Clause, Tele, Term};
use crate::syntax::pat::PatCommon;

use super::super::ERROR_TAKE;
use super::{classify_eqs, LhsState};

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
    let weak_sub = Subst::weaken(Default::default(), DBI(len_pats));
    let with_sub = Default::default();
    let pat_sub = Subst::parallel(
        (lhs.pats.iter().take(len_pats).rev().cloned())
            .map(Term::try_from)
            .map(|t| t.expect(ERROR_TAKE)),
    );
    let param_sub = Subst::compose(Subst::compose(pat_sub, weak_sub), with_sub);
    // TODO: check linearity
    let (classified, tcs) = classify_eqs(tcs, lhs.problem.equations)?;
    debug_assert!(!classified.other_pats.is_empty());
    unimplemented!()
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