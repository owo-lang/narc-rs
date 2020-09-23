use std::iter::once;

use either::Either::{Left, Right};
use voile_util::{
    loc::*,
    tags::{Plicit, VarRec},
    uid::{next_uid, DBI, GI},
};

use crate::{
    check::{
        monad::{TCE, TCM, TCS},
        rules::term::{check, whnf::simplify},
    },
    syntax::{
        abs::Abs,
        core::{subst::DeBruijn, Bind, CodataInfo, DataInfo, Decl, Elim, Term, TermInfo, Val},
    },
};

type InferTCM = TCM<(TermInfo, Term, TCS)>;

/// Infer the type of the expression.
pub fn infer(mut tcs: TCS, input_term: &Abs) -> InferTCM {
    if !tcs.trace_tc {
        return infer_impl(tcs, input_term);
    }
    // Continue with logging
    let depth_ws = tcs.tc_depth_ws();
    tcs.tc_deeper();
    let (evaluated, inferred_ty, mut tcs) = infer_impl(tcs, input_term).map_err(|e| {
        println!("{}Inferring {}", depth_ws, input_term);
        e
    })?;
    println!(
        "{}\u{22A2} {} : {} \u{2191} {}",
        depth_ws, input_term, inferred_ty, evaluated.ast
    );
    tcs.tc_shallower();
    Ok((evaluated, inferred_ty, tcs))
}

fn infer_impl(tcs: TCS, abs: &Abs) -> InferTCM {
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
        let (mut ty_val, mut new_tcs) = simplify(tcs, ty)?;
        match loop {
            let (param, clos) = match ty_val {
                Val::Pi(param, clos) => (param, clos),
                Val::Data(i) if i.kind == VarRec::Record => break Right((i.def, i.args)),
                e => return Err(TCE::NotPi(Term::Whnf(e), arg.loc())),
            };
            let (param_ty, loop_tcs) = simplify(new_tcs, *param.ty)?;
            new_tcs = loop_tcs;
            // In case this is an implicit argument
            if param.licit == Plicit::Im {
                // This meta has type "param_ty".
                let meta = new_tcs.fresh_meta();
                elims.push(Elim::app(meta.clone()));
                let (new_ty_val, loop_tcs) = simplify(new_tcs, clos.instantiate(meta))?;
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
                        Decl::Codata(i) => (i.name.clone(), &i.fields),
                        _ => unreachable!(),
                    };
                    if !codata_fields.iter().any(|(_, &ix)| ix == proj_def) {
                        return Err(TCE::DifferentFieldCodata(
                            ident.loc,
                            codata_name.text.clone(),
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
        Decl::Data(DataInfo {
            loc, params, level, ..
        })
        | Decl::Codata(CodataInfo {
            loc, params, level, ..
        }) => Ok(Term::pi_from_tele(params.clone(), Term::universe(*level)).at(*loc)),
        Decl::Cons(cons) => {
            let params = &cons.params;
            let data = cons.data;
            let data_tele = match tcs.def(data) {
                Decl::Data(i) => &i.params,
                _ => unreachable!(),
            };
            let params_len = params.len();
            let range = params_len..params_len + data_tele.len();
            let tele = data_tele
                .iter()
                .cloned()
                // Because we have no GADT (
                .map(Bind::into_implicit)
                .chain(params.iter().cloned())
                .collect();
            let ident = tcs.def(data).def_name().clone();
            let elims = range.rev().map(DBI).map(Elim::from_dbi).collect();
            let ret = Term::def(data, ident, elims);
            Ok(Term::pi_from_tele(tele, ret).at(cons.loc()))
        }
        Decl::Proj(proj) => {
            let data_tele = match tcs.def(proj.codata) {
                Decl::Codata(i) => &i.params,
                _ => unreachable!(),
            };
            let range = 0..data_tele.len() - 1;
            let ident = tcs.def(proj.codata).def_name().clone();
            let elims = range.rev().map(DBI).map(Elim::from_dbi).collect();
            let codata = Term::def(proj.codata, ident, elims);
            let bind = Bind::new(Plicit::Ex, unsafe { next_uid() }, codata);
            let tele = (data_tele.iter().cloned())
                // Or maybe we shouldn't?
                .map(Bind::into_implicit)
                .chain(once(bind))
                .collect();
            Ok(Term::pi_from_tele(tele, proj.ty.clone()).at(proj.loc()))
        }
        Decl::Func(func) => Ok(func.signature.clone().at(func.loc)),
        Decl::ClausePlaceholder => unreachable!(),
    }
}

fn infer_head(mut tcs: TCS, input_term: &Abs) -> InferTCM {
    if !tcs.trace_tc {
        return infer_head_impl(tcs, input_term);
    }
    // Continue with logging
    let depth_ws = tcs.tc_depth_ws();
    tcs.tc_deeper();
    let (evaluated, inferred_ty, mut tcs) = infer_head_impl(tcs, input_term).map_err(|e| {
        println!("{}Head-inferring {}", depth_ws, input_term);
        e
    })?;
    println!(
        "{}\u{22A2} {} : {} \u{2192} {}",
        depth_ws, input_term, inferred_ty, evaluated.ast
    );
    tcs.tc_shallower();
    Ok((evaluated, inferred_ty, tcs))
}

fn infer_head_impl(tcs: TCS, abs: &Abs) -> InferTCM {
    use Abs::*;
    match abs {
        Proj(id, def) | Cons(id, def) | Def(id, def) => type_of_decl(&tcs, *def)
            .map(|ty| (Term::simple_def(*def, id.clone()).at(id.loc), ty.ast, tcs)),
        Var(loc, var) => {
            let bind = tcs.local_by_id(*var);
            Ok((bind.val.at(loc.loc), bind.bind.ty, tcs))
        }
        e => Err(TCE::NotHead(e.clone())),
    }
}
