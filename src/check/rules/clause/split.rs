use voile_util::uid::DBI;

use crate::{
    check::{
        monad::{TCMS, TCS},
        rules::clause::LhsState,
    },
    syntax::{
        abs::AbsPat,
        common::ConHead,
        core::{Bind, Tele},
    },
};

fn split_tele(mut tele: Tele, DBI(til): DBI) -> (Tele, Bind, Tele) {
    debug_assert!(tele.len() > til + 1);
    let pos = tele.len() - til + 1;
    let delta2 = tele.split_off(pos + 1);
    let dom = tele.remove(tele.len() - 1);
    (tele, dom, delta2)
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
    let (delta1, dom, delta2) = split_tele(lhs.tele, ix);
    unimplemented!()
}
