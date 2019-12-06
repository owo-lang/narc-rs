use std::convert::TryFrom;
use std::rc::Rc;

use voile_util::uid::{DBI, UID};

use crate::check::monad::{TCE, TCMS, TCS};
use crate::check::rules::clause::{AsBind, PatVars};
use crate::check::rules::term::is_eta_var_borrow;
use crate::syntax::abs::Abs;
use crate::syntax::core::subst::{DeBruijn, RedEx, Subst};
use crate::syntax::core::{Pat, Tele, Term};
use crate::syntax::pat::{Copat, PatCommon};

use super::super::ERROR_TAKE;
use super::{classify_eqs, LhsState};

/// Result of checking the LHS of a clause.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.html#LHSResult).
#[derive(Clone)]
pub struct Lhs {
    /// $\Delta$: The types of the pattern variables, in internal dependency order.
    /// Corresponds to `clauseTel` (in Agda).
    pub tele: Tele,
    /// Whether the LHS has at least one absurd pattern.
    pub has_absurd: bool,
    /// The patterns in internal syntax.
    pub pats: Vec<Pat>,
    /// The type of the body. Is $b~\sigma$ if $\Gamma$ is defined.
    pub ty: Term,
    /// Substitution version of `pats`, only up to the first projection pattern.
    /// $\Delta \vdash \text{pat\_subst} : \Gamma$.
    /// Where $\Gamma$ is the argument telescope of the function.
    /// This is used to update inherited dot patterns in
    /// with-function clauses.
    pub pat_sub: Rc<Subst>,
    /// As-bindings from the left-hand side.
    /// Return instead of bound since we
    /// want them in where's and right-hand sides, but not in with-clauses
    pub as_binds: Vec<AsBind>,
}

/// Build a renaming for the internal patterns using variable names from
/// the user patterns. If there are multiple user names for the same internal
/// variable, the unused ones are returned as as-bindings.
/// Names that are not also module parameters are preferred over
/// those that are.
///
/// # Parameters
///
/// + `tele`: The telescope of pattern variables
/// + `pat_vars`: The list of user names for each pattern variable
///
pub fn user_variable_names(tele: &Tele, mut pat_vars: PatVars) -> (Vec<Option<UID>>, Vec<AsBind>) {
    let len_rng = 0..tele.len();
    let mut as_binds = Vec::with_capacity(pat_vars.len());
    let mut names = Vec::with_capacity(tele.len());
    for (bind, ix) in tele.iter().zip(len_rng.rev().map(DBI)) {
        let ids = pat_vars.remove(&ix).unwrap_or_default();
        names.push(ids.first().copied());
        for uid in ids {
            let as_bind = AsBind::new(uid, DeBruijn::from_dbi(ix), bind.ty.clone());
            as_binds.push(as_bind)
        }
    }
    (names, as_binds)
}

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
pub fn final_check(tcs: TCS, mut lhs: LhsState) -> TCMS<Lhs> {
    debug_assert!(lhs.problem.todo_pats.is_empty());
    let len_pats = lhs.len_pats();
    // It should be `len_pats - ctx.len()`,
    // but I think the `ctx` in Agda comes from module parameters | variables | stuff,
    // which we don't really support.
    // let weak_sub = Subst::weaken(Default::default(), DBI(len_pats));
    let pat_sub = Subst::parallel(
        (lhs.pats.iter().take(len_pats).rev().cloned())
            .map(Term::try_from)
            .map(|t| t.expect(ERROR_TAKE)),
    );
    // let with_sub = Default::default();
    // let param_sub = Subst::compose(Subst::compose(pat_sub.clone(), weak_sub), with_sub);
    // TODO: check linearity
    let equations = lhs.problem.equations;
    let (classified, tcs) = tcs.under(&mut lhs.tele, |tcs| classify_eqs(tcs, equations))?;
    debug_assert!(classified.other_pats.is_empty());
    let (vars, mut asb) = user_variable_names(&lhs.tele, classified.pat_vars);
    // The variable name stands for `rename`.
    let ren = Subst::parallel(
        (vars.into_iter().rev())
            .enumerate()
            .map(|(b, _)| DeBruijn::from_dbi(DBI(b))),
    );
    let mut as_binds = classified.as_binds;
    as_binds.append(&mut asb);
    let lhs_result = Lhs {
        tele: lhs.tele,
        has_absurd: classified.absurd_count > 0,
        pats: lhs.pats.reduce_dbi(&ren),
        ty: lhs.target,
        pat_sub,
        as_binds,
    };
    // Agda is calling `computeLHSContext` here, and is updating context with `param_sub`.
    // TODO: do it.
    Ok((lhs_result, tcs))
}

/// Checking a pattern matching lhs recursively.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.html).
pub fn check_lhs(mut tcs: TCS, lhs: LhsState) -> TCMS<Lhs> {
    for split in (lhs.problem.equations.iter()).filter(|e| e.in_pat.is_split()) {
        if lhs.problem.is_all_solved() {
            return final_check(tcs, lhs);
        }
        let (is_eta, tcs0) = is_eta_var_borrow(tcs, &split.inst, &split.ty)?;
        tcs = tcs0;
        let ix = is_eta.ok_or_else(|| TCE::SplitOnNonVar(split.inst.clone(), split.ty.clone()))?;
        let pos = lhs.tele.len() - ix.0 + 1;
        let (delta1, delta2) = lhs.tele.split_at(pos);
        unimplemented!()
    }
    // Really?
    // debug_assert!(lhs.problem.is_all_solved());
    final_check(tcs, lhs)
}
