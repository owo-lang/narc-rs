use voile_util::{tags::VarRec, uid::DBI};

use crate::{
    check::{
        monad::{TCE, TCMS, TCS},
        rules::{clause::state::LhsState, term::expect_data},
    },
    syntax::{
        abs::AbsPat,
        common::ConHead,
        core::{Bind, Decl, Tele, Val::Data},
        pat::Copat,
    },
};

fn split_tele(mut tele: Tele, DBI(til): DBI) -> (Tele, Bind, Tele) {
    debug_assert!(tele.len() > til + 1);
    let pos = tele.len() - til + 1;
    let delta2 = tele.split_off(pos + 1);
    let dom = tele.remove(tele.len() - 1);
    (tele, dom, delta2)
}

/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.html#local-6989586621681883972).
pub(super) fn split_proj(tcs: TCS, lhs: LhsState, proj: String) -> TCMS<LhsState> {
    let (data, tcs) = expect_data(tcs, lhs.target)?;
    if data.kind == VarRec::Variant {
        return Err(TCE::not_codata(Data(data)));
    }
    let rec_info = match tcs.def(data.def) {
        Decl::Codata(info) => info,
        _ => unreachable!(),
    };
    let proj_ix = match rec_info.fields.get(&proj) {
        Some(i) => *i,
        None => return Err(TCE::NoSuchProj(proj)),
    };
    let proj_info = match tcs.def(proj_ix) {
        Decl::Proj(info) => info.clone(),
        _ => unreachable!(),
    };
    let target = proj_info.ty;
    // It might be of a function type taking a `self` parameter,
    // we should apply it to `target`.
    let mut pats = lhs.pats;
    pats.push(Copat::Proj(proj));
    // `lhs.problem.take_first_todo_pat()` has
    // already modified the todo_pat
    let lhs = LhsState {
        pats,
        target,
        ..lhs
    };

    Ok((lhs, tcs))
}

/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.html#local-6989586621683054881).
pub(super) fn split_con(
    tcs: TCS,
    ix: DBI,
    lhs: LhsState,
    is_forced: bool,
    head: ConHead,
    pats: Vec<AbsPat>,
) -> TCMS<LhsState> {
    let (mut delta1, dom, delta2) = split_tele(lhs.tele, ix);
    let (data, tcs) = expect_data(tcs, dom.ty)?;
    // For disambiguation:
    // tcs.under(&mut delta1, |tcs| {
    //     unimplemented!()
    // })?;
    let cons_params = match tcs.def(head.cons_ix) {
        Decl::Cons(c) => c.params.to_vec(),
        _ => unreachable!(),
    };
    // Agda checks if we're splitting on non-eta records as we
    // shouldn't split on lazy (non-eta) constructor.
    unimplemented!()
}
