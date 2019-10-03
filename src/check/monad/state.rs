use crate::syntax::core::{Bind, Decl, Tele, Term};
use voile_util::meta::MetaContext;
use voile_util::uid::{DBI, GI, UID};

/// Typing context.
pub type Sigma = Vec<Decl>;

/// Type-checking state.
#[derive(Debug, Clone, Default)]
pub struct TCS {
    /// Global context (definitions are attached with type annotations).
    pub sigma: Sigma,
    /// Local typing context.
    pub gamma: Tele,
    /// Meta variable context. Always global.
    pub meta_context: MetaContext<Term>,
}

impl TCS {
    /// Create a new valid but unsolved meta variable,
    /// used for generating fresh metas during elaboration.
    pub fn fresh_meta(&mut self) -> Term {
        self.meta_context.fresh_meta(|m| Term::meta(m, vec![]))
    }

    pub fn def(&self, ix: GI) -> &Decl {
        &self.sigma[ix.0]
    }

    pub fn local(&self, ix: DBI) -> &Bind {
        &self.gamma[ix.0]
    }

    pub fn local_by_id(&self, id: UID) -> (DBI, &Bind) {
        self.local_by_id_safe(id).expect("Unresolved reference")
    }

    pub fn local_by_id_safe(&self, id: UID) -> Option<(DBI, &Bind)> {
        (self.gamma.iter().enumerate())
            .find(|(_, b)| b.name == id)
            .map(|(ix, bind)| (DBI(ix), bind))
    }

    pub fn mut_def(&mut self, ix: GI) -> &mut Decl {
        &mut self.sigma[ix.0]
    }

    pub fn mut_local(&mut self, ix: DBI) -> &mut Bind {
        &mut self.gamma[ix.0]
    }
}
