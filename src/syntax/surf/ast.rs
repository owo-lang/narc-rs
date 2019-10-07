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
    /// Explicit meta variable.
    Meta(Ident),
    /// Dot-projection.
    Proj(Ident),
    /// Application
    App(Box<Vec1<Self>>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprDecl {
    Defn(Ident, Expr),
    Cls(Ident, Vec<ExprCopat>, Expr),
}

/// In `ExprPat`, the `ConHead` is pseudo, please beware of this fact and
/// do proper desugar to produce valid abstract syntax.
pub type ExprCopat = Copat<Ident, Expr>;
pub type ExprPat = Pat<Ident, Expr>;
