use voile_util::{loc::Ident, uid::GI};

use crate::{
    check::{
        monad::{ValTCM, TCE, TCM, TCS},
        pats::{build_subst, match_copats, Blocked, Match, RedM, Stuck},
    },
    syntax::{
        common::{ConHead, Ductive},
        core::{subst::RedEx, Clause, Decl, Elim, Term, Val},
    },
};

pub fn simplify(tcs: TCS, term: Term) -> ValTCM {
    match term {
        Term::Whnf(whnf) => Ok((whnf, tcs)),
        Term::Redex(def, id, elims) => match tcs.def(def) {
            Decl::Data(_) => Ok((Val::inductive(def, elims_to_terms(elims)?), tcs)),
            Decl::Codata(_) => Ok((Val::coinductive(def, elims_to_terms(elims)?), tcs)),
            Decl::Cons(cons) => {
                let (ductive, fields) = match &cons.fields {
                    Some(fields) => (Ductive::In, fields.clone()),
                    None => (Ductive::Coin, vec![]),
                };
                let head = ConHead::new(id, cons.data, ductive, fields);
                Ok((Val::Cons(head, elims_to_terms(elims)?), tcs))
            }
            Decl::Proj { .. } => unimplemented!(),
            Decl::Func(func) => {
                let clauses = (func.clauses.iter())
                    // Our elims should be enough
                    .filter(|clause| elims.len() >= clause.patterns.len())
                    // Should not be an absurd clause
                    .filter(|clause| !clause.is_absurd())
                    .cloned()
                    .collect();
                match unfold_func(&tcs, def, id, clauses, elims) {
                    Ok((_, term)) => simplify(tcs, term),
                    Err(blockage) => match blockage.stuck {
                        Stuck::NotBlocked => simplify(tcs, blockage.anyway),
                        _ => Err(TCE::blocked(blockage)),
                    },
                }
            }
            Decl::ClausePlaceholder => unreachable!(),
        },
    }
}

/// Build up a substitution and unfold the declaration.
pub fn unfold_func(
    tcs: &TCS,
    def: GI,
    func_name: Ident,
    clauses: Vec<Clause>,
    mut elims: Vec<Elim>,
) -> RedM<Term, Blocked<Term>> {
    for clause in clauses {
        let mut es = elims;
        let pat_len = clause.patterns.len();
        let mut rest = es.split_off(pat_len);
        let copats = clause.patterns.into_iter().zip(es.into_iter());
        let (m, es) = match_copats(tcs, copats);
        match m {
            Match::Yes(s, vs) => {
                let subst = build_subst(vs, pat_len);
                let body = match clause.body {
                    None => {
                        elims = es;
                        elims.append(&mut rest);
                        let term = Term::def(def, func_name, elims);
                        return Err(Blocked::new(Stuck::AbsurdMatch, term));
                    }
                    Some(body) => body,
                };
                return Ok((s, body.reduce_dbi(subst).apply_elim(rest)));
            }
            Match::Dunno(b) => {
                elims = es;
                elims.append(&mut rest);
                return Err(b.map_anyway(|()| Term::def(def, func_name, elims)));
            }
            // continue to next clause
            Match::No => {
                elims = es;
                elims.append(&mut rest);
            }
        }
    }
    let term = Term::def(def, func_name, elims);
    Err(Blocked::new(Stuck::MissingClauses, term))
}

fn elims_to_terms(elims: Vec<Elim>) -> TCM<Vec<Term>> {
    elims
        .into_iter()
        .map(Elim::try_into_app)
        .collect::<Result<_, String>>()
        .map_err(TCE::NotTerm)
}
