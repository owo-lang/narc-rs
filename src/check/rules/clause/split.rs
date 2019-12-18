use crate::{
    check::{
        monad::{TCMS, TCS},
        rules::clause::LhsState,
    },
    syntax::{abs::AbsPat, common::ConHead},
};

/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Rules.LHS.html#local-6989586621683054881).
pub(super) fn split_con(
    tcs: TCS,
    lhs: LhsState,
    is_forced: bool,
    head: ConHead,
    pats: Vec<AbsPat>,
) -> TCMS<LhsState> {
    unimplemented!()
}
