use voile_util::loc::ToLoc;

use crate::check::monad::{ValTCM, TCE, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::{Term, Val};

use self::infer::*;

mod infer;

pub fn check(tcs: TCS, abs: &Abs, against: &Val) -> ValTCM {
    match (abs, against) {
        (Abs::Type(info, lower), Val::Type(upper)) => {
            if upper > lower {
                Ok((Term::universe(*lower).into_info(info.loc), tcs))
            } else {
                Err(TCE::LevelMismatch(abs.loc(), *lower + 1, *upper))
            }
        }
        (expr, anything) => check_fallback(tcs, expr, anything),
    }
}
