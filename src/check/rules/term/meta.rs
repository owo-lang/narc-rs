use crate::check::monad::{TCE, TCMS, TCS};
use crate::syntax::core::{Elim, Term, Val};

pub trait HasMeta {
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
            Term::Whnf(w) => w.inline_meta(tcs).map(|(w, tcs)| (Term::Whnf(w), tcs)),
            Term::Redex(gi, id, elims) => {
                let (elims, tcs) = elims.inline_meta(tcs)?;
                Ok((Term::Redex(gi, id, elims), tcs))
            }
        }
    }
}

impl HasMeta for Val {
    fn inline_meta(self, tcs: TCS) -> TCMS<Self> {
        use Val::*;
        match self {
            Type(l) => Ok((Type(l), tcs)),
            Data(k, gi, args) => args.inline_meta(tcs).map(|(a, tcs)| (Data(k, gi, a), tcs)),
            Pi(_, _) => unimplemented!(),
            Cons(c, ts) => ts.inline_meta(tcs).map(|(ts, tcs)| (Cons(c, ts), tcs)),
            Meta(_, _) => unimplemented!(),
            Axiom(a) => Ok((Axiom(a), tcs)),
            Var(head, args) => args.inline_meta(tcs).map(|(a, tcs)| (Var(head, a), tcs)),
            Id(t, a, b) => {
                let (t, tcs) = t.inline_meta(tcs)?;
                let (a, tcs) = a.inline_meta(tcs)?;
                let (b, tcs) = b.inline_meta(tcs)?;
                Ok((Term::identity_val(t, a, b), tcs))
            }
            Refl => Ok((Refl, tcs)),
        }
    }
}
