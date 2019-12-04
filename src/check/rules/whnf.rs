use crate::check::monad::{ValTCM, TCE, TCM, TCS};
use crate::syntax::common::Ductive;
use crate::syntax::core::{ConHead, Decl, Elim, Term};

pub fn simplify(tcs: TCS, term: Term) -> ValTCM {
    match term {
        Term::Whnf(whnf) => Ok((whnf, tcs)),
        Term::Redex(def, id, elims) => match tcs.def(def) {
            Decl::Data(_) => simplify(tcs, Term::inductive(def, elims_to_terms(elims)?)),
            Decl::Codata(_) => simplify(tcs, Term::coinductive(def, elims_to_terms(elims)?)),
            Decl::Cons(cons) => {
                let (ductive, fields) = match &cons.fields {
                    Some(fields) => (Ductive::In, fields.clone()),
                    None => (Ductive::Coin, vec![]),
                };
                let head = ConHead::new(id, cons.data, ductive, fields);
                simplify(tcs, Term::cons(head, elims_to_terms(elims)?))
            }
            Decl::Proj { .. } => unimplemented!(),
            // TODO: build up a substitution and unfold the declaration.
            Decl::Func { .. } => unimplemented!(),
            Decl::ClausePlaceholder => unreachable!(),
        },
    }
}

fn elims_to_terms(elims: Vec<Elim>) -> TCM<Vec<Term>> {
    elims
        .into_iter()
        .map(|elim| elim.try_into_app())
        .collect::<Result<_, String>>()
        .map_err(TCE::NotTerm)
}
