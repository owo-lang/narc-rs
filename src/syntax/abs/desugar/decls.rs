use crate::syntax::surf::{ExprDecl, NamedTele};

use super::{desugar_expr, DesugarM, DesugarState};
use crate::syntax::abs::AbsDecl;
use voile_util::loc::ToLoc;

pub fn desugar_decls(state: DesugarState, decls: Vec<ExprDecl>) -> DesugarM {
    decls.into_iter().try_fold(state, desugar_decl)
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
        Cls(name, pats, body) => unimplemented!(),
        Data(signature, conses) => unimplemented!(),
        Codata(signature, fields) => unimplemented!(),
    }
}
