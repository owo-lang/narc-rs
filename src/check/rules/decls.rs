use voile_util::uid::GI;

use crate::{
    check::{
        monad::{TCM, TCS},
        rules::{
            clause::clause,
            data::check_data,
            term::{check, HasMeta},
            ERROR_MSG,
        },
    },
    syntax::{
        abs::AbsDecl,
        core::{Decl, FuncInfo, TYPE_OMEGA},
    },
};

pub fn check_decls(mut tcs: TCS, decls: Vec<AbsDecl>) -> TCM {
    let mut decls = decls.into_iter().map(Some).collect::<Vec<_>>();
    let range = 0..decls.len();
    let take = |decls: &mut [Option<AbsDecl>], i: usize| decls[i].take().expect(ERROR_MSG);

    for i in range {
        if decls[i].is_none() {
            continue;
        }
        let decl = take(&mut decls, i);
        tcs.tc_reset_depth();
        match decl {
            AbsDecl::Data(i) => {
                let cs = (i.conses.iter())
                    .map(|GI(j)| match take(&mut decls, *j) {
                        AbsDecl::Cons(i) => i,
                        _ => unreachable!(ERROR_MSG),
                    })
                    .collect();
                // TODO: Inline meta??
                tcs = check_data(tcs, i, cs)?;
            }
            AbsDecl::Cons(_) => unreachable!(ERROR_MSG),
            AbsDecl::Defn(i) => {
                let (ty, new_tcs) = check(tcs, &i.ty, &TYPE_OMEGA)?;
                let (signature, new_tcs) = ty.ast.inline_meta(new_tcs)?;
                tcs = new_tcs;
                let func = FuncInfo {
                    loc: i.source,
                    name: i.name,
                    signature,
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
                    _ => unreachable!(),
                };
                tcs.sigma.push(Decl::ClausePlaceholder);
            }
            _ => unimplemented!(),
        }
    }
    Ok(tcs)
}
