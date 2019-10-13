use voile_util::vec1::Vec1;

use crate::syntax::abs::{Abs, AbsDecl};
use crate::syntax::surf::Expr;

use super::{DesugarErr, DesugarM, DesugarState};

pub fn desugar_expr(state: DesugarState, expr: Expr) -> DesugarM<(Abs, DesugarState)> {
    match expr {
        Expr::Var(v) => {
            if let Some(uid) = state.lookup_local(&v.text) {
                Ok((Abs::Var(v, uid), state))
            } else if let Some((ix, decl)) = state.lookup_by_name(&v.text) {
                use AbsDecl::*;
                match decl {
                    Codata { .. } | Data { .. } | Defn { .. } => Ok((Abs::Def(v, ix), state)),
                    Cons { .. } => Ok((Abs::Cons(v, ix), state)),
                    // A proj gets applied, using the application syntax
                    // (instead of the dot-projection syntax)
                    Proj { .. } => Ok((Abs::Def(v, ix), state)),
                    Clause { .. } => unreachable!(),
                }
            } else {
                Err(DesugarErr::UnresolvedReference(v.clone()))
            }
        }
        Expr::Type(i) => Ok((Abs::universe(i), state)),
        Expr::Meta(i) => {
            let mut state = state;
            let meta = Abs::meta(i, state.fresh_meta());
            Ok((meta, state))
        }
        Expr::Proj(i) => {
            // TODO: better error msg for resolved but non-proj
            if let Some((ix, AbsDecl::Proj { .. })) = state.lookup_by_name(&i.text) {
                Ok((Abs::Proj(i, ix), state))
            } else {
                Err(DesugarErr::UnresolvedReference(i))
            }
        }
        Expr::App(head, tail) => {
            let (head, state) = desugar_expr(state, *head)?;
            let init = (state, Vec::with_capacity(tail.len()));
            let (state, mut tail) = tail.try_fold(init, |(st, mut v), e| {
                let (e, st) = desugar_expr(st, e)?;
                v.push(e);
                Ok((st, v))
            })?;
            let tail_head = tail.remove(0);
            Ok((Abs::app(head, Vec1::new(tail_head, tail)), state))
        }
        Expr::Pi(_, _) => unimplemented!(),
    }
}
