use crate::check::monad::{TCM, TCS};
use crate::syntax::core::{Term, Val};

pub fn subtype(tcs: TCS, sub: &Val, sup: &Val) -> TCM {
    use Val::*;
    match (sub, sup) {
        (Type(sub_l), Type(sup_l)) if sub_l <= sup_l => Ok(tcs),
        (e, t) => unify_val(tcs, e, t),
    }
}

pub fn unify(tcs: TCS, left: &Term, right: &Term) -> TCM {
    use Term::*;
    match (left, right) {
        (Whnf(left), Whnf(right)) => unify_val(tcs, left, right),
        _ => unimplemented!(),
    }
}

pub fn unify_val(tcs: TCS, left: &Val, right: &Val) -> TCM {
    use Val::*;
    match (left, right) {
        (Type(sub_l), Type(sup_l)) if sub_l == sup_l => Ok(tcs),
        _ => unimplemented!(),
    }
}
