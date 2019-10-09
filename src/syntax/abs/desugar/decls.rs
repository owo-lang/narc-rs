use crate::syntax::surf::ExprDecl;

use super::{DesugarErr, DesugarM, DesugarState};

pub fn desugar_decls(state: DesugarState, decls: Vec<ExprDecl>) -> DesugarM {
    decls.into_iter().try_fold(tcs, desugar_decl)
}

pub fn desugar_decl(state: DesugarState, decl: ExprDecl) -> DesugarM {
    unimplemented!()
}
