use voile_util::meta::MI;

use crate::syntax::core::Elim;

#[derive(Debug, Clone)]
pub enum Stuck {
    /// Blocked by meta.
    OnMeta(MI),
    /// The `Elim` is neutral and blocks a pattern match.
    OnElim(Elim),
    /// Not enough arguments were supplied to complete the matching.
    UnderApplied,
    /// We matched an absurd clause, results in a neutral `Def`.
    AbsurdMatch,
    /// We ran out of clauses, all considered clauses
    /// produced an actual mismatch.
    /// This can happen when try to reduce a function application,
    /// but we are still missing some function clauses.
    /// See `Agda.TypeChecking.Patterns.Match`.
    MissingClauses,
    /// Reduction was not blocked, we reached a whnf
    /// which can be anything, but a stuck `Whnf::Redex`.
    NotBlocked,
}

impl Stuck {
    pub fn is_meta(&self) -> Option<MI> {
        match self {
            Stuck::OnMeta(m) => Some(*m),
            _ => None,
        }
    }
}

/// Something where a meta variable may block reduction.
#[derive(Debug, Clone)]
pub struct Blocked<T> {
    pub stuck: Stuck,
    pub ignore_blocking: T,
}

impl<T> Blocked<T> {
    pub fn is_meta(&self) -> Option<MI> {
        self.stuck.is_meta()
    }
}
