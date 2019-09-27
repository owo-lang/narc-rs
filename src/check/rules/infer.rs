use voile_util::loc::ToLoc;
use voile_util::tags::Plicit;
use voile_util::uid::{DBI, GI};

use crate::check::monad::{TermTCM, TCE, TCS};
use crate::check::rules::check;
use crate::check::rules::whnf::normalize;
use crate::syntax::abs::Abs;
use crate::syntax::core::subst::RedEx;
use crate::syntax::core::{Decl, Elim, Param, Term, Val};

use super::eval::eval;
use super::unify::subtype;

/// Infer the type of an expression.
pub fn infer(tcs: TCS, abs: &Abs) -> TermTCM {
    let abs = match abs {
        Abs::Type(id, level) => return Ok((Term::universe(*level + 1).at(id.loc), tcs)),
        abs => abs.clone(),
    };
    let view = abs.into_app_view();
    let (head, mut tcs) = infer_head(tcs, &view.fun)?;
    let mut ty = head.ast;
    for (loc, arg) in view.args {
        let (param, clos) = match ty {
            Term::Whnf(Val::Pi(param, clos)) => (param, clos),
            e => return Err(TCE::NotPi(e, loc)),
        };
        let (param, new_tcs) = normalize(tcs, *param.term)?;
        let (arg, new_tcs) = check(new_tcs, &arg, &param)?;
        ty = clos.instantiate(arg.ast);
        tcs = new_tcs;
    }
    Ok((ty.at(head.loc), tcs))
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
            let params_len = params.len();
            let range = params_len..params_len + data_tele.len() - 1;
            let tele = data_tele
                .iter()
                .cloned()
                // Because we have no GADT (
                .map(Param::into_implicit)
                .chain(params.iter().cloned())
                .collect();
            let ret = Term::def(*data, range.rev().map(DBI).map(Elim::from_dbi).collect());
            Ok((Term::pi_from_tele(tele, ret).at(*loc), tcs))
        }
        Decl::Proj {
            loc, codata, ty, ..
        } => {
            let data_tele = match tcs.def(*codata) {
                Decl::Codata { params, .. } => params,
                _ => unreachable!(),
            };
            let range = 0..data_tele.len() - 1;
            let codata = Term::def(*codata, range.rev().map(DBI).map(Elim::from_dbi).collect());
            let tele = data_tele
                .iter()
                .cloned()
                // Or maybe we shouldn't?
                .map(Param::into_implicit)
                .chain(vec![Param::new(Plicit::Ex, codata)].into_iter())
                .collect();
            Ok((Term::pi_from_tele(tele, ty.clone()).at(*loc), tcs))
        }
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

pub fn check_fallback(tcs: TCS, expr: Abs, expected_type: &Val) -> TermTCM {
    let (inferred, tcs) = infer(tcs, &expr)?;
    let (whnf, tcs) = normalize(tcs, inferred.ast)?;
    let tcs = subtype(tcs, &whnf, expected_type).map_err(|e| e.wrap(expr.loc()))?;
    eval(tcs, expr)
}
