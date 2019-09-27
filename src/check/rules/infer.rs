use voile_util::loc::ToLoc;
use voile_util::tags::Plicit;
use voile_util::uid::{DBI, GI};

use crate::check::monad::{TermTCM, ValTCM, TCE, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::subst::RedEx;
use crate::syntax::core::{Decl, Elim, Param, Term, Val};

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
        }
        | Decl::Codata {
            loc, params, level, ..
        } => {
            let term = Term::pi_from_tele(params.clone(), Term::universe(*level)).at(*loc);
            Ok((term, tcs))
        }
        Decl::Cons {
            loc, data, params, ..
        } => {
            let data_tele = match tcs.def(*data) {
                Decl::Data { params, .. } => params,
                _ => unreachable!(),
            };
            let data_tele_len = data_tele.len();
            let tele = data_tele
                .iter()
                .cloned()
                .map(Param::into_implicit)
                .chain(params.iter().cloned())
                .collect();
            let params_len = params.len();
            let range = params_len..params_len + data_tele_len;
            let ret = Term::def(*data, range.rev().map(DBI).map(Elim::from_dbi).collect());
            Ok((Term::pi_from_tele(tele, ret).at(*loc), tcs))
        }
        Decl::Proj { .. } => unimplemented!(),
        Decl::Func { loc, signature, .. } => Ok((signature.clone().at(*loc), tcs)),
    }
}

pub fn infer_head(tcs: TCS, abs: &Abs) -> TermTCM {
    use Abs::*;
    match abs {
        Proj(_, def) | Cons(_, def) | Def(_, def) => type_of_decl(tcs, *def),
        Var(loc, var) => {
            let ty: &Param = unimplemented!();
            debug_assert_eq!(ty.licit, Plicit::Ex);
            Ok((ty.term.clone().at(loc.loc), tcs))
        }
        e => Err(TCE::NotHead(e.clone())),
    }
}

pub fn check_fallback(tcs: TCS, expr: &Abs, expected_type: &Val) -> TermTCM {
    let (inferred, tcs) = infer(tcs, expr)?;
    let tcs = subtype(tcs, &inferred, expected_type).map_err(|e| e.wrap(expr.loc()))?;
    eval(tcs, expr.clone())
}
