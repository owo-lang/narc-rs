use voile_util::level::Level;
use voile_util::loc::{Ident, Loc, ToLoc};
use voile_util::uid::GI;

use super::{Abs, AbsCopat, AbsTele};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbsConsInfo {
    pub source: Loc,
    pub name: Ident,
    pub tele: AbsTele,
    /// Corresponding datatype's index.
    pub data_ix: GI,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbsProjInfo {
    pub source: Loc,
    pub name: Ident,
    pub ty: Abs,
    /// Corresponding coinductive record's index.
    pub codata_ix: GI,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbsCodataInfo {
    pub source: Loc,
    pub self_ref: Option<Ident>,
    pub name: Ident,
    pub fields: Vec<GI>,
    pub level: Level,
    pub tele: AbsTele,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbsDataInfo {
    pub source: Loc,
    pub name: Ident,
    pub level: Level,
    pub tele: AbsTele,
    pub conses: Vec<GI>,
}

/// Declaration.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Abstract.html#t:Declaration).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AbsDecl {
    /// Datatypes.
    Data(AbsDataInfo),
    /// Datatype constructors.
    Cons(AbsConsInfo),
    /// Coinductive record projections.
    Proj(AbsProjInfo),
    /// Function signature definition.
    Defn { source: Loc, name: Ident, ty: Abs },
    /// Pattern matching clause.
    Clause(AbsClause),
    /// Coinductive records.
    Codata(AbsCodataInfo),
}

impl AbsDecl {
    pub fn defn(source: Loc, name: Ident, ty: Abs) -> Self {
        AbsDecl::Defn { source, name, ty }
    }

    pub fn decl_name(&self) -> &Ident {
        use AbsDecl::*;
        match self {
            Defn { name, .. } => name,
            Clause(info) => &info.name,
            Data(info) => &info.name,
            Cons(info) => &info.name,
            Proj(info) => &info.name,
            Codata(info) => &info.name,
        }
    }
}

impl ToLoc for AbsDecl {
    fn loc(&self) -> Loc {
        use AbsDecl::*;
        match self {
            Defn { source, .. } => *source,
            Data(i) => i.loc(),
            Cons(i) => i.loc(),
            Clause(i) => i.loc(),
            Codata(i) => i.loc(),
            Proj(i) => i.loc(),
        }
    }
}

/// Clause information in abstract syntax.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.html#Clause%27).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbsClause {
    pub source: Loc,
    /// Name of the function we're adding clause to.
    pub name: Ident,
    /// Lhs.
    pub patterns: Vec<AbsCopat>,
    /// Index of the type signature definition.
    pub definition: GI,
    /// Rhs.
    pub body: Abs,
}
