use voile_util::loc::{Loc, ToLoc};

pub use self::ast::*;
pub use self::ast_fold::*;
pub use self::ast_util::*;
pub use self::decl::*;
pub use self::decl_impl::*;
pub use self::pat::*;
pub use self::pretty::*;

/// Core language syntax definitions.
mod ast;
/// Ast traversal functions.
///
/// I don't think these functions should be used much, though.
/// We should use pattern matching instead.
mod ast_fold;
/// Constructor functions & utility functions.
///
/// To avoid too much `Box::new` invocations.
mod ast_util;
/// Checked declarations.
mod decl;
/// Declarations' trivial trait implementations.
mod decl_impl;
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

/// Telescopes.
pub type Tele = Vec<Bind>;
pub type TeleS = [Bind];
pub type LetList = Vec<Let>;

/// Contexts.
pub type Ctx = Vec<Term>;
