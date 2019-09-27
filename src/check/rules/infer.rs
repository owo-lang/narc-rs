use voile_util::loc::ToLoc;
use voile_util::uid::GI;

use crate::check::monad::{TermTCM, ValTCM, TCE, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::{Decl, Term, Val};

use super::eval::eval;
use super::unify::subtype;

/// Infer the type of an expression.
pub fn infer(tcs: TCS, abs: &Abs) -> ValTCM {
    unimplemented!()
}

pub fn type_of_decl(tcs: TCS, decl: GI) -> TermTCM {
    let decl = tcs.def(decl);
    match decl {
        Decl::Data {
            loc, params, level, ..
        } => {
            let term = Term::pi_from_tele(params.clone(), Term::universe(*level)).at(*loc);
            Ok((term, tcs))
        }
        Decl::Codata { .. } => unimplemented!(),
        Decl::Cons { .. } => unimplemented!(),
        Decl::Proj { .. } => unimplemented!(),
        Decl::Func { loc, signature, .. } => Ok((signature.clone().at(*loc), tcs)),
    }
}

pub fn infer_head(tcs: TCS, abs: &Abs) -> TermTCM {
    match abs {
        Abs::Def(_, def) => type_of_decl(tcs, *def),
        Abs::Var(_, var) => unimplemented!(),
        Abs::App(_, _, _) => unimplemented!(),
        Abs::Cons(_, _) => unimplemented!(),
        Abs::Proj(_, _) => unimplemented!(),
        e => Err(TCE::NotHead(e.clone())),
    }
}

pub fn check_fallback(tcs: TCS, expr: &Abs, expected_type: &Val) -> TermTCM {
    let (inferred, tcs) = infer(tcs, expr)?;
    let tcs = subtype(tcs, &inferred, expected_type).map_err(|e| e.wrap(expr.loc()))?;
    eval(tcs, expr.clone())
}
