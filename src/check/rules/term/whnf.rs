use voile_util::loc::Ident;

use crate::check::monad::{ValTCM, TCE, TCM, TCS};
use crate::check::pats::{build_subst, match_copats, Match};
use crate::check::rules::ERROR_MSG;
use crate::syntax::common::{ConHead, Ductive};
use crate::syntax::core::subst::RedEx;
use crate::syntax::core::{Clause, Decl, Elim, Term, Val};

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
                let term = unfold_func(id, clauses, elims)?;
                simplify(tcs, term)
            }
            Decl::ClausePlaceholder => unreachable!(),
        },
    }
}

/// Build up a substitution and unfold the declaration.
fn unfold_func(f: Ident, clauses: Vec<Clause>, mut elims: Vec<Elim>) -> TCM<Term> {
    for clause in clauses {
        let mut es = elims;
        let pat_len = clause.patterns.len();
        let mut rest = es.split_off(pat_len);
        let (m, es) = match_copats(clause.patterns.into_iter().zip(es.into_iter()));
        match m {
            Match::Yes(s, vs) => {
                let subst = build_subst(vs, pat_len);
                let body = clause.body.expect(ERROR_MSG);
                return if s.into() {
                    Ok(body.reduce_dbi(subst).apply_elim(rest))
                } else {
                    Err(TCE::CantSimplify(f))
                };
            }
            // continue to next clause
            Match::No => {
                elims = es;
                elims.append(&mut rest);
            }
        }
    }
    Err(TCE::CantFindPattern(f))
}

fn elims_to_terms(elims: Vec<Elim>) -> TCM<Vec<Term>> {
    elims
        .into_iter()
        .map(Elim::try_into_app)
        .collect::<Result<_, String>>()
        .map_err(TCE::NotTerm)
}
