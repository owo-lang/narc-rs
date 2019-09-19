use crate::check::monad::{ValTCM, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::Val;

/// Infer the type of an expression.
pub fn infer(tcs: TCS, abs: &Abs) -> ValTCM {
    unimplemented!()
}

pub fn check_fallback(tcs: TCS, expr: &Abs, expected_type: &Val) -> ValTCM {
    let (inferred, tcs) = infer(tcs, expr)?;
    unimplemented!()
}
