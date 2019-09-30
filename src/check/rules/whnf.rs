use crate::check::monad::{ValTCM, TCS};
use crate::syntax::core::Term;

pub fn normalize(tcs: TCS, term: Term) -> ValTCM {
    match term {
        Term::Whnf(whnf) => Ok((whnf, tcs)),
        // TODO: build up a substitution and unfold the declaration.
        Term::Redex(..) => unimplemented!(),
    }
}
