use voile_util::meta::MetaContext;
use voile_util::uid::{DBI, GI, UID};

use crate::syntax::core::subst::{DeBruijn, RedEx, Subst};
use crate::syntax::core::{Bind, Decl, Let, LetList, Tele, Term};
use std::iter::repeat;

/// Typing context.
pub type Sigma = Vec<Decl>;

const UNRESOLVED: &str = "Unresolved reference";

/// Type-checking state.
#[derive(Debug, Clone, Default)]
pub struct TCS {
    tc_depth: usize,
    /// Global context (definitions are attached with type annotations).
    pub sigma: Sigma,
    /// Local typing context.
    pub gamma: Tele,
    /// Let bindings.
    pub lets: LetList,
    /// Meta variable context. Always global.
    pub meta_context: MetaContext<Term>,
}

impl TCS {
    /// For debugging purpose.
    pub fn tc_depth_ws(&self) -> String {
        repeat(' ').take(self.tc_depth).collect()
    }

    pub fn tc_deeper(&mut self) {
        self.tc_depth += 1;
    }

    pub fn tc_shallower(&mut self) {
        if self.tc_depth > 0 {
            self.tc_depth -= 1;
        }
    }

    pub fn tc_reset_depth(&mut self) {
        self.tc_depth = 0;
    }

    pub fn reserve_local_variables(&mut self, additional: usize) {
        self.gamma.reserve(additional);
        self.sigma.reserve(additional);
    }

    /// Create a new valid but unsolved meta variable,
    /// used for generating fresh metas during elaboration.
    pub fn fresh_meta(&mut self) -> Term {
        self.meta_context.fresh_meta(|m| Term::meta(m, vec![]))
    }

    pub fn def(&self, ix: GI) -> &Decl {
        &self.sigma[ix.0]
    }

    pub fn local(&self, ix: DBI) -> &Bind {
        let len = self.gamma.len();
        &self.gamma[len - ix.0 - 1]
    }

    pub fn local_by_id(&self, id: UID) -> Let {
        self.local_by_id_safe(id).expect(UNRESOLVED)
    }

    pub fn local_by_id_safe(&self, id: UID) -> Option<Let> {
        let lookup_let = || self.let_by_id_safe(id).cloned();
        let lookup_gamma = || {
            let (i, ty) = self.gamma_by_id_safe(id)?;
            let ty = ty.clone().reduce_dbi(&Subst::raise(i + 1));
            Some(Let::new(ty, DeBruijn::from_dbi(i)))
        };
        lookup_let().or_else(lookup_gamma)
    }

    fn let_by_id_safe(&self, id: UID) -> Option<&Let> {
        self.lets.iter().find(|b| b.bind.name == id)
    }

    fn gamma_by_id_safe(&self, id: UID) -> Option<(DBI, &Bind)> {
        let gamma_len = self.gamma.len();
        (self.gamma.iter().enumerate())
            .find(|(_, b)| b.name == id)
            .map(|(ix, bind)| (DBI(gamma_len - ix - 1), bind))
    }

    pub fn mut_def(&mut self, ix: GI) -> &mut Decl {
        &mut self.sigma[ix.0]
    }

    pub fn mut_local(&mut self, ix: DBI) -> &mut Bind {
        &mut self.gamma[ix.0]
    }
}
