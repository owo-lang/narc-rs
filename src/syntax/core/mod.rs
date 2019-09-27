use voile_util::loc::{Loc, ToLoc};

pub use self::ast::*;
pub use self::ast_cons::*;
pub use self::decl::*;
pub use self::pat::*;
pub use self::pretty::*;

/// Core language syntax definitions.
mod ast;
/// Constructor functions.
mod ast_cons;
/// Checked declarations.
mod decl;
/// Patterns and its operations.
mod pat;
/// Pretty printing things.
mod pretty;
/// Substitution is a mapping.
pub mod subst;

impl Term {
    pub fn at(self, loc: Loc) -> TermInfo {
        TermInfo::new(self, loc)
    }
}

/// A value with syntax info.
/// This is what should be stored inside of the context.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TermInfo {
    pub ast: Term,
    pub loc: Loc,
}

impl TermInfo {
    pub fn new(ast: Term, loc: Loc) -> Self {
        Self { ast, loc }
    }

    pub fn map_ast(self, f: impl FnOnce(Term) -> Term) -> Self {
        Self::new(f(self.ast), self.loc)
    }
}

impl ToLoc for TermInfo {
    fn loc(&self) -> Loc {
        self.loc.clone()
    }
}

/// Telescopes.
pub type Tele = Vec<Param>;

/// Contexts.
pub type Ctx = Vec<Term>;
