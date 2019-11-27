use crate::syntax::core::{Closure, Elim, Term, Val};

pub trait FoldVal {
    fn try_fold_val<E, R>(
        &self,
        init: R,
        f: impl Fn(R, &Val) -> Result<R, E> + Copy,
    ) -> Result<R, E>;
}

impl<T: FoldVal> FoldVal for [T] {
    fn try_fold_val<E, R>(
        &self,
        init: R,
        f: impl Fn(R, &Val) -> Result<R, E> + Copy,
    ) -> Result<R, E> {
        self.iter().try_fold(init, |a, v| v.try_fold_val(a, f))
    }
}

impl FoldVal for Term {
    fn try_fold_val<E, R>(
        &self,
        init: R,
        f: impl Fn(R, &Val) -> Result<R, E> + Copy,
    ) -> Result<R, E> {
        use Term::*;
        match self {
            Whnf(val) => val.try_fold_val(init, f),
            Redex(_, _, args) => args.try_fold_val(init, f),
        }
    }
}

impl FoldVal for Elim {
    fn try_fold_val<E, R>(
        &self,
        init: R,
        f: impl Fn(R, &Val) -> Result<R, E> + Copy,
    ) -> Result<R, E> {
        match self {
            Elim::App(a) => a.try_fold_val(init, f),
            Elim::Proj(..) => Ok(init),
        }
    }
}

impl FoldVal for Closure {
    fn try_fold_val<E, R>(
        &self,
        init: R,
        f: impl Fn(R, &Val) -> Result<R, E> + Copy,
    ) -> Result<R, E> {
        match self {
            Closure::Plain(t) => t.try_fold_val(init, f),
        }
    }
}

impl FoldVal for Val {
    fn try_fold_val<E, R>(
        &self,
        init: R,
        f: impl Fn(R, &Val) -> Result<R, E> + Copy,
    ) -> Result<R, E> {
        use Val::*;
        let init = f(init, self)?;
        match self {
            Data(_, _, v) | Cons(_, v) => v.try_fold_val(init, f),
            Axiom(..) | Type(..) | Refl => Ok(init),
            Pi(p, clos) => clos.try_fold_val(p.ty.try_fold_val(init, f)?, f),
            Id(a, b, c) => c.try_fold_val(b.try_fold_val(a.try_fold_val(init, f)?, f)?, f),
            Var(_, v) | Meta(_, v) => v.try_fold_val(init, f),
        }
    }
}
