use voile_util::loc::{Ident, ToLoc};
use voile_util::uid::GI;

use crate::syntax::abs::{Abs, AbsDecl, AbsTele};
use crate::syntax::surf::{Expr, ExprCopat, ExprDecl, NamedTele};

use super::{desugar_expr, DesugarErr, DesugarM, DesugarState};

pub fn desugar_decls(state: DesugarState, decls: Vec<ExprDecl>) -> DesugarM {
    decls.into_iter().try_fold(state, desugar_decl)
}

pub fn desugar_signature(
    state: DesugarState,
    signature: NamedTele,
) -> DesugarM<(Ident, AbsTele, DesugarState)> {
    unimplemented!()
}

pub fn desugar_clause(
    state: DesugarState,
    defn_ix: GI,
    name: Ident,
    pats: Vec<ExprCopat>,
    body: Expr,
) -> DesugarM {
    unimplemented!()
}

pub fn desugar_decl(state: DesugarState, decl: ExprDecl) -> DesugarM {
    use ExprDecl::*;
    match decl {
        Defn(name, sig) => {
            let (sig, mut state) = desugar_expr(state, sig)?;
            let abs_decl = AbsDecl::defn(name.loc + sig.loc(), name, sig);
            state.decls.push(abs_decl);
            Ok(state)
        }
        Cls(name, pats, body) => match state.lookup_by_name(&name.text) {
            Some((ix, AbsDecl::Defn { .. })) => desugar_clause(state, ix, name, pats, body),
            None => {
                let mut state = state;
                let meta = Abs::Meta(name.clone(), state.fresh_meta());
                let decl_len = state.decl_len();
                let mut state = desugar_clause(state, decl_len, name.clone(), pats, body)?;
                let defn = AbsDecl::defn(name.loc, name, meta);
                state.decls.push(defn);
                Ok(state)
            }
            Some((_, other)) => Err(DesugarErr::NotDefn(other.decl_name().clone())),
        },
        Data(signature, conses) => {
            let (name, tele, mut state) = desugar_signature(state, signature)?;
            state.decls.reserve(conses.len());
            unimplemented!()
        }
        Codata(signature, fields) => {
            let (name, tele, mut state) = desugar_signature(state, signature)?;
            state.decls.reserve(fields.len());
            unimplemented!()
        }
    }
}
