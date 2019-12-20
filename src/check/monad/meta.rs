use std::rc::Rc;

use voile_util::{meta::MI, uid::DBI};

use crate::syntax::core::subst::{RedEx, Subst};
use std::fmt::Debug;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MetaSol<Val> {
    /// Solved meta, solved at .
    ///
    /// Boxed to make the variable smaller.
    Solved(DBI, Box<Val>),
    /// Not yet solved meta.
    Unsolved,
}

impl<Val> Default for MetaSol<Val> {
    fn default() -> Self {
        MetaSol::Unsolved
    }
}

impl<R, T: RedEx<R>> RedEx<MetaSol<R>> for MetaSol<T> {
    fn reduce_dbi(self, subst: Rc<Subst>) -> MetaSol<R> {
        use MetaSol::*;
        match self {
            Solved(i, t) => MetaSol::solved(i, t.reduce_dbi(subst)),
            Unsolved => Unsolved,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MetaContext<Val>(Vec<MetaSol<Val>>);

impl<Val> Default for MetaContext<Val> {
    fn default() -> Self {
        MetaContext(Vec::new())
    }
}

impl<Val> MetaSol<Val> {
    pub fn solved(at: DBI, val: Val) -> Self {
        MetaSol::Solved(at, Box::new(val))
    }
}

impl<Val> MetaContext<Val> {
    pub fn solutions(&self) -> &Vec<MetaSol<Val>> {
        &self.0
    }

    pub fn solution(&self, index: MI) -> &MetaSol<Val> {
        &self.solutions()[index.0]
    }

    pub fn mut_solutions(&mut self) -> &mut Vec<MetaSol<Val>> {
        &mut self.0
    }

    /// Add many unsolved metas to the context.
    pub fn expand_with_fresh_meta(&mut self, meta_count: MI) {
        debug_assert!(self.solutions().len() <= meta_count.0);
        self.mut_solutions()
            .resize_with(meta_count.0, Default::default);
    }

    /// Create a new valid but unsolved meta variable,
    /// used for generating fresh metas during elaboration.
    pub fn fresh_meta(&mut self, new_meta: impl FnOnce(MI) -> Val) -> Val {
        let meta = new_meta(MI(self.solutions().len()));
        self.mut_solutions().push(MetaSol::Unsolved);
        meta
    }
}

impl<Val: Debug + Eq> MetaContext<Val> {
    /// Submit a solution to a meta variable to the context.
    pub fn solve_meta(&mut self, meta_index: MI, at: DBI, solution: Val) {
        let meta_solution = &mut self.mut_solutions()[meta_index.0];
        debug_assert_eq!(meta_solution, &mut MetaSol::Unsolved);
        *meta_solution = MetaSol::solved(at, solution);
    }
}
