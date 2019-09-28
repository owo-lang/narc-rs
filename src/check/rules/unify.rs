use crate::check::monad::{TCE, TCM, TCS};
use crate::syntax::core::{Elim, Term, Val};

pub fn subtype(tcs: TCS, sub: &Val, sup: &Val) -> TCM {
    use Val::*;
    match (sub, sup) {
        (Type(sub_l), Type(sup_l)) if sub_l <= sup_l => Ok(tcs),
        (e, t) => unify_val(tcs, e, t),
    }
}

pub fn unify(mut tcs: TCS, left: &Term, right: &Term) -> TCM {
    use Term::*;
    match (left, right) {
        (Whnf(left), Whnf(right)) => unify_val(tcs, left, right),
        (Redex(g0, args0), Redex(g1, args1)) if args0.len() == args1.len() => {
            if g0 != g1 {
                return Err(TCE::DifferentName(
                    tcs.def(*g0).def_name().clone(),
                    tcs.def(*g1).def_name().clone(),
                ));
            }
            for (a, b) in args0.iter().zip(args1.iter()) {
                tcs = unify_elim(tcs, a, b)?;
            }
            Ok(tcs)
        }
        (a, b) => Err(TCE::DifferentTerm(a.clone(), b.clone())),
    }
}

pub fn unify_elim(tcs: TCS, left: &Elim, right: &Elim) -> TCM {
    use Elim::*;
    match (left, right) {
        (Proj(a), Proj(b)) if a == b => Ok(tcs),
        (App(a), App(b)) => unify(tcs, &**a, &**b),
        (a, b) => Err(TCE::DifferentElim(a.clone(), b.clone())),
    }
}

pub fn unify_val(tcs: TCS, left: &Val, right: &Val) -> TCM {
    use Val::*;
    match (left, right) {
        (Type(sub_l), Type(sup_l)) if sub_l == sup_l => Ok(tcs),
        _ => unimplemented!(),
    }
}
