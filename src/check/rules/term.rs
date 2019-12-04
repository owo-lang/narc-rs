use voile_util::loc::ToLoc;
use voile_util::uid::DBI;

use crate::check::monad::{TermTCM, TCE, TCMS, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::{Bind, Closure, Term, Val};

use super::infer::*;
use super::unify::subtype;
use super::whnf::simplify;

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
                .push(Bind::new(bind.licit, bind.name, bind_ty.ast, None));
            let (ret_ty, mut tcs) = check(tcs, &**ret, against)?;
            let bind_ty = tcs.gamma.pop().expect("Bad index");
            let term = Term::pi2(bind_ty.boxed(), Closure::plain(ret_ty.ast));
            Ok((term.at(*info), tcs))
        }
        (expr, anything) => check_fallback(tcs, expr.clone(), anything),
    }
}

pub fn check_fallback(tcs: TCS, expr: Abs, expected_type: &Val) -> TermTCM {
    let (evaluated, inferred, tcs) = infer(tcs, &expr)?;
    let (whnf, tcs) = simplify(tcs, inferred)?;
    let tcs = subtype(tcs, &whnf, expected_type).map_err(|e| e.wrap(expr.loc()))?;
    Ok((evaluated, tcs))
}

/// A borrowing version of [`is_eta_var`](Self::is_eta_var).
pub fn is_eta_var_borrow(tcs: TCS, term: &Term, ty: &Term) -> TCMS<Option<DBI>> {
    match term {
        Term::Whnf(Val::Var(dbi, v)) if v.is_empty() => Ok((Some(*dbi), tcs)),
        _ => is_eta_var(tcs, term.clone(), ty.clone()),
    }
}

/// Checks whether the given term (of the given type) is beta-eta-equivalent
/// to a variable. Returns just the de Bruijn-index of the variable if it is,
/// or nothing otherwise.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Records.html#isEtaVar).
///
/// TODO: type-directedness.
pub fn is_eta_var(tcs: TCS, term: Term, ty: Term) -> TCMS<Option<DBI>> {
    let (term, tcs) = simplify(tcs, term)?;
    let (ty, tcs) = simplify(tcs, ty)?;
    match (term, ty) {
        (Val::Var(dbi, v), _) if v.is_empty() => Ok((Some(dbi), tcs)),
        _ => unimplemented!(),
    }
}
