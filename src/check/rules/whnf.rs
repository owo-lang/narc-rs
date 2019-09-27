use crate::check::monad::{ValTCM, TCS};
use crate::syntax::core::Term;

pub fn normalize(tcs: TCS, term: Term) -> ValTCM {
    match term {
        Term::Whnf(whnf) => Ok((whnf, tcs)),
        Term::Redex(..) => unimplemented!(),
    }
}
