use voile_util::loc::Ident;

use crate::check::monad::{ValTCM, TCE, TCM, TCS};
use crate::syntax::common::Ductive;
use crate::syntax::core::{ConHead, Decl, Elim, Term};

pub fn normalize(tcs: TCS, term: Term) -> ValTCM {
    match term {
        Term::Whnf(whnf) => Ok((whnf, tcs)),
        Term::Redex(def, elims) => match tcs.def(def) {
            Decl::Data { .. } => normalize(tcs, Term::inductive(def, elims_to_terms(elims)?)),
            Decl::Codata { .. } => normalize(tcs, Term::coinductive(def, elims_to_terms(elims)?)),
            Decl::Cons {
                loc,
                name,
                data,
                fields,
                ..
            } => {
                let (ductive, fields) = match fields {
                    Some(fields) => (Ductive::In, fields.clone()),
                    None => (Ductive::Coin, vec![]),
                };
                let ident = Ident {
                    loc: *loc,
                    text: name.clone(),
                };
                // How can I find the real name ident?
                let head = ConHead::new(ident, *data, ductive, fields);
                normalize(tcs, Term::cons(head, elims_to_terms(elims)?))
            }
            Decl::Proj { .. } => unimplemented!(),
            // TODO: build up a substitution and unfold the declaration.
            Decl::Func { .. } => unimplemented!(),
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
