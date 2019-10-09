use voile_util::loc::Ident;
use voile_util::tags::Plicit;
use voile_util::vec1::Vec1;

use crate::syntax::pat::{Copat, Pat};

/// Surface syntax: Parameter.
///
/// It's a part of a pi-type expression.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Param {
    pub licit: Plicit,
    /// This field can be empty -- which indicates the parameter to be anonymous.
    /// Many `name`s means there are many params with same type.
    pub names: Vec<Ident>,
    /// Parameter type.
    pub ty: Expr,
}

/// Surface syntax elements.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    /// Variable reference
    Var(Ident),
    /// Universe.
    Type(Ident),
    /// Explicit meta variable.
    Meta(Ident),
    /// Dot-projection.
    Proj(Ident),
    /// Application, chained.
    App(Box<Vec1<Self>>),
    /// Pi-type expression, where `a -> b -> c` is represented as `Pi(vec![a, b], c)`
    /// instead of `Pi(a, Pi(b, c))`.
    /// `a` and `b` here can introduce telescopes.
    Pi(Vec<Param>, Box<Self>),
}

impl Expr {
    pub fn pi(params: Vec<Param>, expr: Self) -> Self {
        Expr::Pi(params, Box::new(expr))
    }

    pub fn app(applied: Self, arguments: Vec<Self>) -> Self {
        Expr::App(Box::new(Vec1::new(applied, arguments)))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprDecl {
    Defn(Ident, Expr),
    Cls(Ident, Vec<ExprCopat>, Expr),
    Data(NamedTele, Vec<ExprCons>),
    Codata(NamedTele, Vec<ExprProj>),
}

pub type ExprCons = NamedTele;
pub type ExprProj = NamedTele;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NamedTele {
    name: Ident,
    tele: Vec<Param>,
}

impl NamedTele {
    pub fn new(name: Ident, tele: Vec<Param>) -> Self {
        Self { name, tele }
    }
}

/// In `ExprPat`, the `ConHead` is pseudo, please beware of this fact and
/// do proper desugar to produce valid abstract syntax.
pub type ExprCopat = Copat<Ident, Expr>;
pub type ExprPat = Pat<Ident, Expr>;
