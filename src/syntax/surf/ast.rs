use voile_util::{
    loc::{Ident, Labelled},
    tags::Plicit,
    vec1::Vec1,
};

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
    App(Box<Self>, Box<Vec1<Self>>),
    /// Pi-type expression, where `a -> b -> c` is represented as `Pi(vec![a, b], c)`
    /// instead of `Pi(a, Pi(b, c))`.
    /// `a` and `b` here can introduce telescopes.
    Pi(Box<Vec1<Param>>, Box<Self>),
}

impl Expr {
    pub fn pi(params: Vec1<Param>, expr: Self) -> Self {
        Expr::Pi(Box::new(params), Box::new(expr))
    }

    pub fn pi_smart(mut params: Vec<Param>, expr: Self) -> Self {
        if params.is_empty() {
            expr
        } else {
            let hd = params.remove(0);
            Self::pi(Vec1::new(hd, params), expr)
        }
    }

    pub fn app(applied: Self, arguments: Vec1<Self>) -> Self {
        Expr::App(Box::new(applied), Box::new(arguments))
    }

    pub fn app_smart(fun: Self, mut args: Vec<Self>) -> Self {
        if args.is_empty() {
            fun
        } else {
            let hd = args.remove(0);
            Self::app(fun, Vec1::new(hd, args))
        }
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
pub type ExprProj = Labelled<Expr>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NamedTele {
    pub name: Ident,
    pub tele: Vec<Param>,
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
