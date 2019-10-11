use voile_util::loc::{Ident, ToLoc};

use crate::syntax::abs::{AbsDecl, AbsTele};
use crate::syntax::surf::{ExprDecl, NamedTele};

use super::{desugar_expr, DesugarM, DesugarState};
use crate::syntax::abs::desugar::error::DesugarErr;

pub fn desugar_decls(state: DesugarState, decls: Vec<ExprDecl>) -> DesugarM {
    decls.into_iter().try_fold(state, desugar_decl)
}

pub fn desugar_signature(
    state: DesugarState,
    signature: NamedTele,
) -> DesugarM<(Ident, AbsTele, DesugarState)> {
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
        Cls(name, pats, body) => {
            match state
                .lookup_by_name(&name.text)
                .ok_or_else(|| DesugarErr::UnresolvedReference(name.clone()))?
            {
                AbsDecl::Defn { .. } => unimplemented!(),
                _ => Err(DesugarErr::NotDefn(name)),
            }
        }
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
