use crate::check::monad::{TCMS, TCS};
use crate::check::rules::term::check;
use crate::check::rules::whnf::simplify;
use crate::syntax::abs::AbsClause;
use crate::syntax::core::{Clause, Tele, Term};

pub use self::eqs::*;
pub use self::lhs::*;
pub use self::state::*;

mod eqs;
mod lhs;
mod state;

/// Bind as patterns
pub fn bind_as_and_tele<T>(
    mut tcs: TCS,
    as_binds: Vec<AsBind>,
    mut tele: Tele,
    f: impl FnOnce(TCS) -> TCMS<T>,
) -> TCMS<T> {
    if tcs.lets.len() < as_binds.len() {
        tcs.lets.reserve(as_binds.len() - tcs.lets.len());
    }
    for bind in as_binds {
        tcs.lets.push(bind.into());
    }
    std::mem::swap(&mut tcs.gamma, &mut tele);
    let (thing, mut tcs) = f(tcs)?;
    tcs.lets.clear();
    std::mem::swap(&mut tcs.gamma, &mut tele);
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
    bind_as_and_tele(tcs, lhs.as_binds, pat_tele.clone(), |mut tcs| {
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
