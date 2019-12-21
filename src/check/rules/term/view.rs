use voile_util::uid::DBI;

use crate::{
    check::{
        monad::{TCMS, TCS},
        rules::simplify,
    },
    syntax::core::{Term, Val},
};

/// A borrowing version of [`is_eta_var`](Self::is_eta_var).
pub fn is_eta_var_borrow(tcs: TCS, term: &Term, ty: &Term) -> TCMS<Option<DBI>> {
    match term {
        Term::Whnf(Val::Var(dbi, v)) if v.is_empty() => Ok((Some(*dbi), tcs)),
        _ => is_eta_var(tcs, term.clone(), ty.clone()),
    }
}

/// Checks whether the given term (of the given type) is beta-eta-equivalent
/// to a variable. Returns just the de Bruijn-index of the variable if it is,
/// or nothing otherwise.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Records.html#isEtaVar).
///
/// TODO: type-directedness.
pub fn is_eta_var(tcs: TCS, term: Term, ty: Term) -> TCMS<Option<DBI>> {
    let (term, tcs) = simplify(tcs, term)?;
    let (ty, tcs) = simplify(tcs, ty)?;
    match (term, ty) {
        (Val::Var(dbi, v), _) if v.is_empty() => Ok((Some(dbi), tcs)),
        _ => unimplemented!(),
    }
}
