use std::hint::unreachable_unchecked;

use voile_util::uid::GI;

use crate::check::monad::{TCM, TCS};
use crate::check::rules::clause::clause;
use crate::check::rules::data::check_data;
use crate::check::rules::term::check;
use crate::syntax::abs::AbsDecl;
use crate::syntax::core::{Decl, FuncInfo, TYPE_OMEGA};

use super::ERROR_TAKE;

pub fn check_decls(mut tcs: TCS, decls: Vec<AbsDecl>) -> TCM {
    let mut decls = decls.into_iter().map(Some).collect::<Vec<_>>();
    let range = 0..decls.len();
    let take = |decls: &mut [Option<AbsDecl>], i: usize| decls[i].take().expect(ERROR_TAKE);

    for i in range {
        if decls[i].is_none() {
            continue;
        }
        let decl = take(&mut decls, i);
        match decl {
            AbsDecl::Data(i) => {
                let cs = (i.conses.iter())
                    .map(|GI(j)| match take(&mut decls, *j) {
                        AbsDecl::Cons(i) => i,
                        _ => unreachable!(ERROR_TAKE),
                    })
                    .collect();
                tcs = check_data(tcs, i, cs)?;
            }
            AbsDecl::Cons(_) => unreachable!(ERROR_TAKE),
            AbsDecl::Defn(i) => {
                let (ty, new_tcs) = check(tcs, &i.ty, &TYPE_OMEGA)?;
                tcs = new_tcs;
                let func = FuncInfo {
                    loc: i.source,
                    name: i.name,
                    signature: ty.ast,
                    clauses: Vec::with_capacity(2),
                };
                tcs.sigma.push(Decl::Func(func));
            }
            AbsDecl::Clause(i) => {
                let def_ix = i.definition;
                let signature = match tcs.def(def_ix) {
                    Decl::Func(f) => f.signature.clone(),
                    _ => unreachable!(),
                };
                let (cls, new_tcs) = clause(tcs, i, signature)?;
                tcs = new_tcs;
                match tcs.mut_def(def_ix) {
                    Decl::Func(f) => f.clauses.push(cls),
                    _ => unsafe { unreachable_unchecked() },
                };
                tcs.sigma.push(Decl::ClausePlaceholder);
            }
            _ => unimplemented!(),
        }
    }
    Ok(tcs)
}
