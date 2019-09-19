use voile_util::loc::ToLoc;

use crate::check::monad::{TermTCM, TCE, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::{Term, Val};

use self::infer::*;
use self::unify::*;

/// Synthesize the type from an abstract term.
mod infer;
/// Conversion check.
mod unify;

pub fn check(tcs: TCS, abs: &Abs, against: &Val) -> TermTCM {
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
