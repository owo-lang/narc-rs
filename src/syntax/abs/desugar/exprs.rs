use voile_util::loc::ToLoc;

use crate::syntax::{
    abs::{
        desugar::{desugar_params, DesugarErr, DesugarM, DesugarState},
        Abs, AbsDecl,
    },
    surf::Expr,
};

pub fn desugar_expr(state: DesugarState, expr: Expr) -> DesugarM<(Abs, DesugarState)> {
    match expr {
        Expr::Var(v) => {
            if let Some(uid) = state.lookup_local(&v.text) {
                Ok((Abs::Var(v, uid), state))
            } else if let Some((ix, decl)) = state.lookup_by_name(&v.text) {
                use AbsDecl::*;
                match decl {
                    Codata { .. } | Data(_) | Defn { .. } => Ok((Abs::Def(v, ix), state)),
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
            let (state, args) = tail.try_scan(state, desugar_expr)?;
            Ok((Abs::app(head, args), state))
        }
        Expr::Pi(params, ret) => {
            let (tele, state) = desugar_params(state, params.into_vec())?;
            let (ret, state) = desugar_expr(state, *ret)?;
            let pi = tele.into_iter().rfold(ret, |ret, bind| {
                let loc = bind.ty.loc() + ret.loc();
                Abs::Pi(loc, bind.boxed(), Box::new(ret))
            });
            Ok((pi, state))
        }
    }
}
