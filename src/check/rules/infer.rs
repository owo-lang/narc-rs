use crate::check::eval::eval;
use crate::check::monad::{TermTCM, ValTCM, TCS};
use crate::check::rules::unify::subtype;
use crate::syntax::abs::Abs;
use crate::syntax::core::Val;
use voile_util::loc::ToLoc;

/// Infer the type of an expression.
pub fn infer(tcs: TCS, abs: &Abs) -> ValTCM {
    unimplemented!()
}

pub fn check_fallback(tcs: TCS, expr: &Abs, expected_type: &Val) -> TermTCM {
    let (inferred, tcs) = infer(tcs, expr)?;
    let tcs = subtype(tcs, &inferred, expected_type).map_err(|e| e.wrap(expr.loc()))?;
    Ok(eval(tcs, expr.clone()))
}
