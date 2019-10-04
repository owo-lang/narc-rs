use voile_util::loc::ToLoc;

use crate::check::monad::{TermTCM, TCE, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::{Bind, Closure, Term, Val};

use self::eval::eval;
use self::infer::*;
use self::unify::subtype;
use self::whnf::normalize;

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
        (Abs::Pi(info, bind, ret), Val::Type(..)) => {
            // Because `against` is `Val::Type(level)`
            let (bind_ty, mut tcs) = check(tcs, &*bind.ty, against)?;
            tcs.gamma
                .push(Bind::new(bind.licit, bind.name, bind_ty.ast));
            let (ret_ty, mut tcs) = check(tcs, &**ret, against)?;
            let bind_ty = tcs.gamma.pop().expect("Bad index");
            let term = Term::pi2(bind_ty.boxed(), Closure::plain(ret_ty.ast));
            Ok((term.at(*info), tcs))
        }
        (expr, anything) => check_fallback(tcs, expr.clone(), anything),
    }
}

pub fn check_fallback(tcs: TCS, expr: Abs, expected_type: &Val) -> TermTCM {
    let (inferred, tcs) = infer(tcs, &expr)?;
    let (whnf, tcs) = normalize(tcs, inferred.ast)?;
    let tcs = subtype(tcs, &whnf, expected_type).map_err(|e| e.wrap(expr.loc()))?;
    eval(tcs, expr)
}
