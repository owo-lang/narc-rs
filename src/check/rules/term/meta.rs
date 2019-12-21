use voile_util::{meta::MI, uid::DBI};

use crate::{
    check::{
        monad::{MetaContext, MetaSol, TCE, TCMS, TCS},
        rules::term::simplify,
    },
    syntax::{
        common::Bind,
        core::{Closure, Elim, Term, Val, ValData},
    },
};

/// For debugging
pub(in crate::check) fn print_meta_ctx(meta: &MetaContext<Term>) {
    use MetaSol::*;
    let solutions = meta.solutions();
    print!("[");
    let mut iter = solutions.iter().enumerate();
    if let Some((ix, sol)) = iter.next() {
        print!("?{:?}", ix);
        if let Solved(DBI(i), sol) = sol {
            print!("={}({:?})", sol, i)
        }
    }
    for (ix, sol) in iter {
        print!(", ?{:?}", ix);
        match sol {
            Solved(DBI(i), sol) => print!("={}({:?})", sol, i),
            Unsolved => print!(","),
        }
    }
    print!("]");
}

/// Somehow like
/// [this](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Reduce.html#Instantiate)
/// in Agda, but different (Agda's instantiates one meta, but this one
/// instantiates the term fully. Maybe this corresponds to `instantiateFull`?).
pub trait HasMeta: Sized {
    /// Inline solved metas inside `self`.
    fn inline_meta(self, tcs: TCS) -> TCMS<Self>;
}

impl<T: HasMeta> HasMeta for Vec<T> {
    fn inline_meta(self, mut tcs: TCS) -> TCMS<Self> {
        let mut r = Vec::with_capacity(self.len());
        for t in self {
            let (t, new_tcs) = t.inline_meta(tcs)?;
            tcs = new_tcs;
            r.push(t);
        }
        Ok((r, tcs))
    }
}

impl HasMeta for ValData {
    fn inline_meta(self, tcs: TCS) -> TCMS<Self> {
        let (args, tcs) = self.args.inline_meta(tcs)?;
        Ok((ValData::new(self.kind, self.def, args), tcs))
    }
}

impl HasMeta for Elim {
    fn inline_meta(self, tcs: TCS) -> TCMS<Self> {
        match self {
            Elim::App(a) => a.inline_meta(tcs).map(|(a, tcs)| (Elim::app(a), tcs)),
            Elim::Proj(p) => Ok((Elim::Proj(p), tcs)),
        }
    }
}

impl HasMeta for Term {
    fn inline_meta(self, tcs: TCS) -> TCMS<Self> {
        match self {
            // Prefer not to simplify
            Term::Whnf(Val::Meta(mi, elims)) => solve_meta(tcs, mi, elims),
            Term::Whnf(w) => w.inline_meta(tcs).map(|(w, tcs)| (Term::Whnf(w), tcs)),
            Term::Redex(gi, id, elims) => {
                let (elims, tcs) = elims.inline_meta(tcs)?;
                Ok((Term::Redex(gi, id, elims), tcs))
            }
        }
    }
}

impl<T: HasMeta> HasMeta for Bind<T> {
    fn inline_meta(self, tcs: TCS) -> TCMS<Self> {
        let (ty, tcs) = self.ty.inline_meta(tcs)?;
        Ok((Bind::new(self.licit, self.name, ty), tcs))
    }
}

impl HasMeta for Closure {
    fn inline_meta(self, mut tcs: TCS) -> TCMS<Self> {
        tcs.unify_depth += 1;
        let (closure, mut tcs) = match self {
            Closure::Plain(body) => {
                let (body, tcs) = body.inline_meta(tcs)?;
                (Closure::plain(body), tcs)
            }
        };
        tcs.unify_depth -= 1;
        Ok((closure, tcs))
    }
}

fn solve_meta(tcs: TCS, mi: MI, elims: Vec<Elim>) -> TCMS<Term> {
    use MetaSol::*;
    let (_ix, sol) = match tcs.meta_ctx().solution(mi) {
        Solved(ix, sol) => (*ix, sol.clone()),
        Unsolved => return Err(TCE::MetaUnsolved(mi)),
    };
    let (elims, tcs) = elims.inline_meta(tcs)?;
    Ok((sol.apply_elim(elims), tcs))
}

impl HasMeta for Val {
    fn inline_meta(self, tcs: TCS) -> TCMS<Self> {
        use Val::*;
        match self {
            Type(l) => Ok((Type(l), tcs)),
            Data(info) => info.inline_meta(tcs).map(|(i, tcs)| (Data(i), tcs)),
            Pi(t, clos) => {
                let (t, tcs) = t.unboxed().inline_meta(tcs)?;
                let (clos, tcs) = clos.inline_meta(tcs)?;
                Ok((Val::Pi(t.boxed(), clos), tcs))
            }
            Cons(c, ts) => ts.inline_meta(tcs).map(|(ts, tcs)| (Cons(c, ts), tcs)),
            Meta(mi, elims) => {
                let (sol, tcs) = solve_meta(tcs, mi, elims)?;
                simplify(tcs, sol)
            }
            Axiom(a) => Ok((Axiom(a), tcs)),
            Var(head, args) => args.inline_meta(tcs).map(|(a, tcs)| (Var(head, a), tcs)),
            Id(t, a, b) => {
                let (t, tcs) = t.inline_meta(tcs)?;
                let (a, tcs) = a.inline_meta(tcs)?;
                let (b, tcs) = b.inline_meta(tcs)?;
                Ok((Val::identity(t, a, b), tcs))
            }
            Refl => Ok((Refl, tcs)),
        }
    }
}
