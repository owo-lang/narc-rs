use voile_util::meta::MI;

use crate::syntax::abs::AbsDecl;

use super::DesugarErr;

/// Desugar Monad.
pub type DesugarM<State = DesugarState> = Result<State, DesugarErr>;

#[derive(Debug, Clone, Default)]
pub struct DesugarState {
    pub decls: Vec<AbsDecl>,
    pub meta_count: MI,
}

impl DesugarState {
    pub fn with_capacity(decl_possible_size: usize) -> Self {
        Self {
            meta_count: Default::default(),
            decls: Vec::with_capacity(decl_possible_size),
        }
    }

    pub fn lookup_decls(&self, filter: impl Fn(&AbsDecl) -> bool) -> Option<&AbsDecl> {
        self.decls.iter().find(|x| filter(*x))
    }

    pub fn lookup_by_name(&self, name: &str) -> Option<&AbsDecl> {
        self.lookup_decls(|decl| name == &decl.decl_name().text)
    }

    pub fn fresh_meta(&mut self) -> MI {
        let ret = self.meta_count;
        self.meta_count += 1;
        ret
    }
}
