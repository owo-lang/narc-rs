use crate::syntax::pat::Pat;

/// The abstract syntax.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Abs {
    TODO, // TODO
}

/// Telescopes in the abstract syntax.
pub type AbsTele = Vec<Abs>;

/// Patterns in the abstract syntax.
pub type AbsPat = Pat<Abs>;
