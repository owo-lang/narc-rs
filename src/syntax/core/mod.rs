use voile_util::loc::{Loc, ToLoc};

pub use self::ast::*;
pub use self::ast_cons::*;
pub use self::pretty::*;
pub use self::redex::*;

/// Core language syntax definitions.
mod ast;
/// Constructor functions.
mod ast_cons;
mod pretty;
/// Reduction function (red-ex stands for **red**ducible **ex**pression).
mod redex;

impl Term {
    pub fn into_info(self, loc: Loc) -> ValInfo {
        ValInfo::new(self, loc)
    }
}

/// A value with syntax info.
/// This is what should be stored inside of the context.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValInfo {
    pub ast: Term,
    pub loc: Loc,
}

impl ValInfo {
    pub fn new(ast: Term, loc: Loc) -> Self {
        Self { ast, loc }
    }

    pub fn map_ast(self, f: impl FnOnce(Term) -> Term) -> Self {
        Self::new(f(self.ast), self.loc)
    }
}

impl ToLoc for ValInfo {
    fn loc(&self) -> Loc {
        self.loc.clone()
    }
}
