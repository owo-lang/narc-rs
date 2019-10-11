use crate::syntax::abs::Abs;
use crate::syntax::surf::Expr;

use super::{DesugarErr, DesugarM, DesugarState};

pub fn desugar_expr(state: DesugarState, expr: Expr) -> DesugarM<(Abs, DesugarState)> {
    unimplemented!()
}
