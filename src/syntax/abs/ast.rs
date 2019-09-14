use crate::syntax::pat::Copat;

/// The abstract syntax.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Abs {
    TODO, // TODO
}

/// Telescopes in the abstract syntax.
pub type AbsTele = Vec<Abs>;

/// Patterns in the abstract syntax.
pub type AbsPat = Copat<Abs>;
