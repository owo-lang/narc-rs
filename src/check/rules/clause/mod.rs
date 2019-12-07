use crate::check::monad::{TCMS, TCS};
use crate::syntax::abs::AbsClause;
use crate::syntax::core::{Clause, Term};

pub use self::eqs::*;
pub use self::lhs::*;
pub use self::state::*;
use crate::check::rules::term::check;
use crate::check::rules::whnf::simplify;

mod eqs;
mod lhs;
mod state;

/// Bind as patterns
pub fn bind_as_pats<T>(mut tcs: TCS, asb: Vec<AsBind>, f: impl FnOnce(TCS) -> TCMS<T>) -> TCMS<T> {
    let init_len = tcs.gamma.len();
    for bind in asb {
        tcs.gamma.push(bind.into());
    }
    let (thing, mut tcs) = f(tcs)?;
    tcs.gamma.split_off(init_len);
    Ok((thing, tcs))
}

/// Checking an abstract clause.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.Def.html#checkClause).
pub fn clause(tcs: TCS, cls: AbsClause, against: Term) -> TCMS<Clause> {
    let body = cls.body;
    // Expand pattern synonyms here once we support it.
    let lhs_state = init_lhs_state(cls.patterns, against)?;
    let (lhs, tcs) = check_lhs(tcs, lhs_state)?;
    let pat_tele = lhs.tele;
    let ty = lhs.ty;
    let patterns = lhs.pats;
    let has_absurd = lhs.has_absurd;
    bind_as_pats(tcs, lhs.as_binds, |mut tcs| {
        let body = if has_absurd {
            None
        } else {
            let (ty, new_tcs) = simplify(tcs, ty)?;
            let (term, new_tcs) = check(new_tcs, &body, &ty)?;
            tcs = new_tcs;
            Some(term.ast)
        };
        let clause = Clause {
            pat_tele,
            patterns,
            body,
        };
        Ok((clause, tcs))
    })
}
