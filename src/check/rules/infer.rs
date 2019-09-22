use voile_util::loc::ToLoc;

use crate::check::monad::{TermTCM, ValTCM, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::Val;

use super::eval::eval;
use super::unify::subtype;

/// Infer the type of an expression.
pub fn infer(tcs: TCS, abs: &Abs) -> ValTCM {
    unimplemented!()
}

pub fn check_fallback(tcs: TCS, expr: &Abs, expected_type: &Val) -> TermTCM {
    let (inferred, tcs) = infer(tcs, expr)?;
    let tcs = subtype(tcs, &inferred, expected_type).map_err(|e| e.wrap(expr.loc()))?;
    eval(tcs, expr.clone())
}
