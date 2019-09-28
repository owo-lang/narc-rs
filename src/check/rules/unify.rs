use crate::check::monad::{TCE, TCM, TCS};
use crate::syntax::core::{Closure, Elim, Term, Val};
use voile_util::uid::GI;

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
        (Redex(i, a), Redex(j, b)) if a.len() == b.len() => {
            tcs = unify_glob_indices(tcs, *i, *j)?;
            unify_elims(tcs, a, b)
        }
        (a, b) => Err(TCE::DifferentTerm(a.clone(), b.clone())),
    }
}

fn unify_glob_indices(tcs: TCS, g0: GI, g1: GI) -> TCM {
    if g0 != g1 {
        return Err(TCE::DifferentName(
            tcs.def(g0).def_name().clone(),
            tcs.def(g1).def_name().clone(),
        ));
    } else {
        Ok(tcs)
    }
}

fn unify_elims(mut tcs: TCS, args0: &[Elim], args1: &[Elim]) -> TCM {
    for (a, b) in args0.iter().zip(args1.iter()) {
        tcs = unify_elim(tcs, a, b)?;
    }
    Ok(tcs)
}

fn unify_terms(mut tcs: TCS, terms0: &[Term], terms1: &[Term]) -> TCM {
    for (a, b) in terms0.iter().zip(terms1.iter()) {
        tcs = unify(tcs, a, b)?;
    }
    Ok(tcs)
}

pub fn unify_elim(tcs: TCS, left: &Elim, right: &Elim) -> TCM {
    use Elim::*;
    match (left, right) {
        (Proj(a), Proj(b)) if a == b => Ok(tcs),
        (App(a), App(b)) => unify(tcs, &**a, &**b),
        (a, b) => Err(TCE::DifferentElim(a.clone(), b.clone())),
    }
}

pub fn unify_closure(tcs: TCS, left: &Closure, right: &Closure) -> TCM {
    use Closure::*;
    match (left, right) {
        (Plain(a), Plain(b)) => unify(tcs, &**a, &**b),
    }
}

pub fn unify_val(mut tcs: TCS, left: &Val, right: &Val) -> TCM {
    use Val::*;
    match (left, right) {
        (Type(sub_l), Type(sup_l)) if sub_l == sup_l => Ok(tcs),
        (Data(k0, i, a), Data(k1, j, b)) if k0 == k1 => {
            tcs = unify_glob_indices(tcs, *i, *j)?;
            unify_terms(tcs, a, b)
        }
        (Pi(a, c0), Pi(b, c1)) if a.licit == b.licit => {
            tcs = unify(tcs, &*a.term, &*b.term)?;
            unify_closure(tcs, c0, c1)
        }
        (Cons(c0, a), Cons(c1, b)) if c0.name == c1.name => unify_terms(tcs, a, b),
        (Axiom(i), Axiom(j)) if i == j => Ok(tcs),
        (Meta(i, a), Meta(j, b)) if i == j => unify_elims(tcs, a, b),
        (Meta(i, a), b) | (b, Meta(i, a)) if a.is_empty() => {
            // TODO: check solution
            tcs.meta_context.solve_meta(*i, Term::Whnf(b.clone()));
            Ok(tcs)
        }
        (App(i, a), App(j, b)) if i == j => unify_elims(tcs, a, b),
        (Id(a, b, c), Id(x, y, z)) => {
            tcs = unify(tcs, &**a, &**x)?;
            tcs = unify(tcs, &**b, &**y)?;
            unify(tcs, &**c, &**z)
        }
        // Uniqueness of identity proof??
        (Refl, Refl) => Ok(tcs),
        (a, b) => Err(TCE::DifferentTerm(
            Term::Whnf(a.clone()),
            Term::Whnf(b.clone()),
        )),
    }
}
