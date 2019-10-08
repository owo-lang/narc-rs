use std::iter::once;

use either::Either::{Left, Right};
use voile_util::loc::ToLoc;
use voile_util::tags::{Plicit, VarRec};
use voile_util::uid::{next_uid, DBI, GI};

use crate::check::monad::{TCE, TCM, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::subst::RedEx;
use crate::syntax::core::{Bind, Decl, Elim, Term, TermInfo, Val};

use super::check;
use super::whnf::normalize;

pub type InferTCM = TCM<(TermInfo, Term, TCS)>;

/// Infer the type of an expression.
pub fn infer(tcs: TCS, abs: &Abs) -> InferTCM {
    let abs = match abs {
        Abs::Type(id, level) => {
            let me = Term::universe(*level).at(id.loc);
            return Ok((me, Term::universe(*level + 1), tcs));
        }
        abs => abs.clone(),
    };
    let view = abs.into_app_view();
    let (head, mut ty, mut tcs) = infer_head(tcs, &view.fun)?;
    let mut elims = Vec::with_capacity(view.args.len());
    for arg in view.args {
        let (mut ty_val, mut new_tcs) = normalize(tcs, ty)?;
        match loop {
            let (param, clos) = match ty_val {
                Val::Pi(param, clos) => (param, clos),
                Val::Data(VarRec::Record, codata_def, elims) => break Right((codata_def, elims)),
                e => return Err(TCE::NotPi(Term::Whnf(e), arg.loc())),
            };
            let (param_ty, loop_tcs) = normalize(new_tcs, *param.ty)?;
            new_tcs = loop_tcs;
            // In case this is an implicit argument
            if param.licit == Plicit::Im {
                // This meta has type "param_ty".
                let meta = new_tcs.fresh_meta();
                elims.push(Elim::app(meta.clone()));
                let (new_ty_val, loop_tcs) = normalize(new_tcs, clos.instantiate(meta))?;
                ty_val = new_ty_val;
                new_tcs = loop_tcs;
            } else {
                break Left((param_ty, clos));
            }
        } {
            Left((param, clos)) => {
                let (arg, new_tcs) = check(new_tcs, &arg, &param)?;
                ty = clos.instantiate(arg.ast.clone());
                elims.push(Elim::app(arg.ast));
                tcs = new_tcs;
            }
            // FIXME: we should take this `_codata_elims` into account when inferring the
            //  type of `proj_def` because in `type_of_decl` we're assuming no type params known
            Right((codata_def, _codata_elims)) => match arg {
                Abs::Proj(ident, proj_def) => {
                    let (codata_name, codata_fields) = match new_tcs.def(codata_def) {
                        Decl::Codata { name, fields, .. } => (name, fields),
                        _ => unreachable!(),
                    };
                    if !codata_fields.contains(&proj_def) {
                        return Err(TCE::FieldCodataMismatch(
                            ident.loc,
                            codata_name.clone(),
                            ident.text,
                        ));
                    }
                    elims.push(Elim::Proj(ident.text));
                    ty = type_of_decl(&new_tcs, proj_def)?.ast;
                    tcs = new_tcs;
                }
                e => return Err(TCE::NotProj(e)),
            },
        }
    }
    Ok((head.map_ast(|t| t.apply_elim(elims)), ty, tcs))
}

pub fn type_of_decl(tcs: &TCS, decl: GI) -> TCM<TermInfo> {
    let decl = tcs.def(decl);
    match decl {
        Decl::Data {
            loc, params, level, ..
        }
        | Decl::Codata {
            loc, params, level, ..
        } => Ok(Term::pi_from_tele(params.clone(), Term::universe(*level)).at(*loc)),
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
                .map(Bind::into_implicit)
                .chain(params.iter().cloned())
                .collect();
            let ret = Term::def(*data, range.rev().map(DBI).map(Elim::from_dbi).collect());
            Ok(Term::pi_from_tele(tele, ret).at(*loc))
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
                .map(Bind::into_implicit)
                .chain(once(Bind::new(Plicit::Ex, unsafe { next_uid() }, codata)))
                .collect();
            Ok(Term::pi_from_tele(tele, ty.clone()).at(*loc))
        }
        Decl::Func { loc, signature, .. } => Ok(signature.clone().at(*loc)),
    }
}

pub fn infer_head(tcs: TCS, abs: &Abs) -> InferTCM {
    use Abs::*;
    match abs {
        Proj(id, def) | Cons(id, def) | Def(id, def) => {
            type_of_decl(&tcs, *def).map(|ty| (Term::simple_def(*def).at(id.loc), ty.ast, tcs))
        }
        Var(loc, var) => {
            let (ix, ty) = tcs.local_by_id(*var);
            Ok((Term::from_dbi(ix).at(loc.loc), ty.ty.clone(), tcs))
        }
        e => Err(TCE::NotHead(e.clone())),
    }
}
