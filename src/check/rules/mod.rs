use voile_util::loc::ToLoc;

use crate::check::monad::{TermTCM, TCE, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::{Term, Val};

use self::clause::clause;
use self::infer::*;

/// Type check a function clause.
mod clause;
/// Turning an abstract term into a core term.
mod eval;
/// Synthesize the type from an abstract term.
mod infer;
/// Conversion check.
mod unify;
/// Find the weak-head-normal-form (normalize) of an expression.
mod whnf;

pub fn check(tcs: TCS, abs: &Abs, against: &Val) -> TermTCM {
    match (abs, against) {
        (Abs::Type(info, lower), Val::Type(upper)) => {
            if upper > lower {
                Ok((Term::universe(*lower).at(info.loc), tcs))
            } else {
                Err(TCE::LevelMismatch(abs.loc(), *lower + 1, *upper))
            }
        }
        (Abs::Pi(info, bind, ret), Val::Type(level)) => {
            // Because `against` is `Val::Type(level)`
            let (tcs, bind_ty) = check(tcs, &*bind.ty, against)?;
            unimplemented!()
        }
        (expr, anything) => check_fallback(tcs, expr.clone(), anything),
    }
}
