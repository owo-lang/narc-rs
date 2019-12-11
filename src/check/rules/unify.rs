use voile_util::meta::{MetaSolution, MI};
use voile_util::uid::{DBI, GI};

use crate::check::monad::{TCE, TCM, TCS};
use crate::check::rules::whnf::simplify;
use crate::syntax::core::subst::{RedEx, Subst};
use crate::syntax::core::{Closure, Elim, FoldVal, Term, Val};

fn check_solution(meta: MI, rhs: &Val) -> TCM<()> {
    rhs.try_fold_val((), |(), v| match v {
        Val::Meta(mi, ..) if mi == &meta => Err(TCE::MetaRecursion(*mi)),
        _ => Ok(()),
    })
}

pub fn subtype(mut tcs: TCS, sub: &Val, sup: &Val) -> TCM {
    if !tcs.trace_tc {
        return subtype_impl(tcs, sub, sup);
    }
    // Continue with logging
    let depth_ws = tcs.tc_depth_ws();
    tcs.tc_deeper();
    let mut tcs = subtype_impl(tcs, sub, sup).map_err(|e| {
        println!("{}Subtyping {} <: {}", depth_ws, sub, sup);
        e
    })?;
    println!("{}\u{22A2} {} <: {}", depth_ws, sub, sup);
    tcs.tc_shallower();
    Ok(tcs)
}

fn subtype_impl(mut tcs: TCS, sub: &Val, sup: &Val) -> TCM {
    use Val::*;
    match (sub, sup) {
        (Type(sub_l), Type(sup_l)) if sub_l <= sup_l => Ok(tcs),
        (Pi(a, c0), Pi(b, c1)) if a.licit == b.licit => {
            tcs = Unify::unify(tcs, &a.ty, &b.ty)?;
            compare_closure(tcs, c0, c1, |tcs, a, b| match (a, b) {
                // Covariance
                (Term::Whnf(left), Term::Whnf(right)) => subtype(tcs, left, right),
                (a, b) => Unify::unify(tcs, a, b),
            })
        }
        (e, t) => Unify::unify(tcs, e, t),
    }
}

pub trait Unify {
    /// Conversion check, maybe can solve metas.
    fn unify(tcs: TCS, left: &Self, right: &Self) -> TCM;
}

impl<T: Unify> Unify for [T] {
    fn unify(mut tcs: TCS, left: &Self, right: &Self) -> TCM {
        for (a, b) in left.iter().zip(right.iter()) {
            tcs = Unify::unify(tcs, a, b)?;
        }
        Ok(tcs)
    }
}

impl<T: Unify> Unify for Box<T> {
    fn unify(tcs: TCS, left: &Self, right: &Self) -> TCM {
        Unify::unify(tcs, &**left, &**right)
    }
}

impl Unify for Term {
    fn unify(mut tcs: TCS, left: &Self, right: &Self) -> TCM {
        use Term::*;
        match (left, right) {
            (Whnf(left), Whnf(right)) => Unify::unify(tcs, left, right),
            (Redex(i, _, a), Redex(j, _, b)) if a.len() == b.len() => {
                tcs = Unify::unify(tcs, i, j)?;
                Unify::unify(tcs, a.as_slice(), b.as_slice())
            }
            (a, b) => Err(TCE::different_term(a.clone(), b.clone())),
        }
    }
}

impl Unify for GI {
    fn unify(tcs: TCS, left: &Self, right: &Self) -> TCM {
        if left != right {
            let left_name = tcs.def(*left).def_name().clone();
            let right_name = tcs.def(*right).def_name().clone();
            Err(TCE::DifferentName(left_name, right_name))
        } else {
            Ok(tcs)
        }
    }
}

impl Unify for Elim {
    fn unify(tcs: TCS, left: &Self, right: &Self) -> TCM {
        use Elim::*;
        match (left, right) {
            (Proj(a), Proj(b)) if a == b => Ok(tcs),
            (App(a), App(b)) => Unify::unify(tcs, a, b),
            (a, b) => Err(TCE::different_elim(a.clone(), b.clone())),
        }
    }
}

fn compare_closure(
    mut tcs: TCS,
    left: &Closure,
    right: &Closure,
    term_cmp: impl FnOnce(TCS, &Term, &Term) -> TCM,
) -> TCM {
    use Closure::*;
    let mut backup = tcs.meta_context.solutions().to_vec();
    let mut meta_ctx = Vec::with_capacity(tcs.meta_context.solutions().len());
    meta_ctx.append(tcs.meta_context.mut_solutions());
    meta_ctx = meta_ctx.reduce_dbi(Subst::raise(DBI(1)));
    tcs.meta_context.mut_solutions().append(&mut meta_ctx);
    tcs = match (left, right) {
        (Plain(a), Plain(b)) => term_cmp(tcs, &**a, &**b)?,
    };
    tcs.meta_context.mut_solutions().clear();
    tcs.meta_context.mut_solutions().append(&mut backup);
    Ok(tcs)
}

impl Unify for Closure {
    fn unify(tcs: TCS, left: &Self, right: &Self) -> TCM {
        compare_closure(tcs, left, right, Unify::unify)
    }
}

impl Unify for Val {
    fn unify(tcs: TCS, left: &Self, right: &Self) -> TCM {
        unify_val(tcs, left, right)
    }
}

fn unify_meta_with(mut tcs: TCS, term: &Val, mi: MI) -> TCM {
    match &tcs.meta_context.solution(mi) {
        MetaSolution::Unsolved => {
            check_solution(mi, term)?;
            tcs.meta_context.solve_meta(mi, Term::Whnf(term.clone()));
            Ok(tcs)
        }
        MetaSolution::Solved(solution) => {
            let solution = *solution.clone();
            let (val, tcs) = simplify(tcs, solution)?;
            Unify::unify(tcs, &val, term)
        }
        MetaSolution::Inlined => unreachable!(),
    }
}

#[allow(clippy::many_single_char_names)]
fn unify_val(mut tcs: TCS, left: &Val, right: &Val) -> TCM {
    use Val::*;
    match (left, right) {
        (Type(sub_l), Type(sup_l)) if sub_l == sup_l => Ok(tcs),
        (Data(k0, i, a), Data(k1, j, b)) if k0 == k1 => {
            tcs = Unify::unify(tcs, i, j)?;
            Unify::unify(tcs, a.as_slice(), b.as_slice())
        }
        (Pi(a, c0), Pi(b, c1)) if a.licit == b.licit => {
            tcs = Unify::unify(tcs, &a.ty, &b.ty)?;
            Unify::unify(tcs, c0, c1)
        }
        (Cons(c0, a), Cons(c1, b)) if c0.name == c1.name => {
            Unify::unify(tcs, a.as_slice(), b.as_slice())
        }
        (Axiom(i), Axiom(j)) if i == j => Ok(tcs),
        (Meta(i, a), Meta(j, b)) if i == j => Unify::unify(tcs, a.as_slice(), b.as_slice()),
        (Meta(i, a), b) | (b, Meta(i, a)) if a.is_empty() => unify_meta_with(tcs, b, *i),
        (Var(i, a), Var(j, b)) if i == j => Unify::unify(tcs, a.as_slice(), b.as_slice()),
        (Id(a, b, c), Id(x, y, z)) => {
            tcs = Unify::unify(tcs, a, x)?;
            tcs = Unify::unify(tcs, b, y)?;
            Unify::unify(tcs, c, z)
        }
        // Uniqueness of identity proof??
        (Refl, Refl) => Ok(tcs),
        (a, b) => Err(TCE::different_term(
            Term::Whnf(a.clone()),
            Term::Whnf(b.clone()),
        )),
    }
}
