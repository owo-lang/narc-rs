use voile_util::loc::ToLoc;

use crate::{
    check::monad::{TermTCM, TCE, TCS},
    syntax::{
        abs::Abs,
        core::{Bind, Closure, Term, Val},
    },
};

pub use self::{
    infer::{infer, type_of_decl},
    meta::HasMeta,
    unify::subtype,
    view::{is_eta_var, is_eta_var_borrow},
    whnf::simplify,
};

/// Synthesize the type and its well-typed form from an abstract term.
mod infer;
/// Solves meta variables inside a term and things.
mod meta;
/// Conversion check.
mod unify;
/// Is a term an eta var? Is it a data or record?
mod view;
/// Find the weak-head-normal-form (semi-normalization) of an expression.
/// TODO: Unfolds declarations.
mod whnf;

pub fn check(mut tcs: TCS, input_term: &Abs, against: &Val) -> TermTCM {
    if !tcs.trace_tc {
        return check_impl(tcs, input_term, against);
    }
    // Continue with logging
    let depth_ws = tcs.tc_depth_ws();
    tcs.tc_deeper();
    let (a, mut tcs) = check_impl(tcs, input_term, against).map_err(|e| {
        println!("{}Checking {} : {}", depth_ws, input_term, against);
        e
    })?;
    println!(
        "{}\u{22A2} {} : {} \u{2193} {}",
        depth_ws, input_term, against, a.ast
    );
    tcs.tc_shallower();
    Ok((a, tcs))
}

fn check_impl(tcs: TCS, abs: &Abs, against: &Val) -> TermTCM {
    match (abs, against) {
        (Abs::Type(info, lower), Val::Type(upper)) => {
            if upper > lower {
                Ok((Term::universe(*lower).at(info.loc), tcs))
            } else {
                Err(TCE::DifferentLevel(abs.loc(), *lower + 1, *upper))
            }
        }
        (Abs::Pi(info, bind, ret), Val::Type(..)) => {
            // Because `against` is `Val::Type(level)`
            let (bind_ty, mut tcs) = check(tcs, &*bind.ty, against)?;
            let new = Bind::new(bind.licit, bind.name, bind_ty.ast);
            tcs.gamma.push(new);
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
