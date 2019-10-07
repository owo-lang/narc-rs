use crate::check::monad::{ValTCM, TCE, TCM, TCS};
use crate::syntax::core::{Decl, Elim, Term};

pub fn normalize(tcs: TCS, term: Term) -> ValTCM {
    match term {
        Term::Whnf(whnf) => Ok((whnf, tcs)),
        Term::Redex(def, elims) => match tcs.def(def) {
            Decl::Data { .. } => normalize(tcs, Term::inductive(def, elims_to_terms(elims)?)),
            Decl::Codata { .. } => normalize(tcs, Term::coinductive(def, elims_to_terms(elims)?)),
            Decl::Cons { .. } => unimplemented!(),
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
