use crate::{
    check::{
        monad::{TCMS, TCS},
        rules::{
            clause::{
                eqs::AsBind,
                lhs::check_lhs,
                state::{progress_lhs_state, LhsState},
            },
            term::{check, simplify, HasMeta},
        },
    },
    syntax::{
        abs::AbsClause,
        core::{Clause, Tele, Term},
    },
};

mod eqs;
mod lhs;
mod split;
mod state;

/// Bind as patterns
fn bind_as_and_tele<T>(
    mut tcs: TCS,
    as_binds: Vec<AsBind>,
    mut tele: Tele,
    f: impl FnOnce(TCS) -> TCMS<T>,
) -> TCMS<T> {
    use std::mem::swap;
    if tcs.lets.len() < as_binds.len() {
        tcs.lets.reserve(as_binds.len() - tcs.lets.len());
    }
    for bind in as_binds {
        tcs.lets.push(bind.into());
    }
    swap(&mut tcs.gamma, &mut tele);
    let (thing, mut tcs) = f(tcs)?;
    tcs.lets.clear();
    swap(&mut tcs.gamma, &mut tele);
    Ok((thing, tcs))
}

/// Checking an abstract clause.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.Def.html#checkClause).
pub fn clause(mut tcs: TCS, cls: AbsClause, against: Term) -> TCMS<Clause> {
    if !tcs.trace_tc {
        return clause_impl(tcs, cls, against);
    }
    // Continue with logging
    let depth_ws = tcs.tc_depth_ws();
    tcs.tc_deeper();
    let cls_name = cls.name.text.clone();
    let (clause, mut tcs) = clause_impl(tcs, cls, against).map_err(|e| {
        println!("{}Clause {}", depth_ws, cls_name);
        e
    })?;
    // Print patterns?
    match &clause.body {
        None => println!("{}\u{22A2} clause {} ()", depth_ws, cls_name,),
        Some(t) => println!("{}\u{22A2} clause {} = {}", depth_ws, cls_name, t),
    }
    tcs.tc_shallower();
    Ok((clause, tcs))
}

fn clause_impl(tcs: TCS, cls: AbsClause, against: Term) -> TCMS<Clause> {
    let body = cls.body;
    // Expand pattern synonyms here once we support it.
    let lhs_state = progress_lhs_state(LhsState::new(cls.patterns, against))?;
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
            let (term, new_tcs) = term.ast.inline_meta(new_tcs)?;
            tcs = new_tcs;
            Some(term)
        };
        let clause = Clause {
            pat_tele,
            patterns,
            body,
        };
        Ok((clause, tcs))
    })
}
