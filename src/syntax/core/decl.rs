use voile_util::level::Level;
use voile_util::loc::*;
use voile_util::uid::GI;

use crate::syntax::core::Pat;

use super::{Tele, Term};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CodataInfo {
    pub loc: Loc,
    pub self_ref: Option<Ident>,
    pub name: Ident,
    pub params: Tele,
    /// References to its projections (fields).
    pub fields: Vec<GI>,
    pub level: Level,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConsInfo {
    pub loc: Loc,
    pub name: Ident,
    pub params: Tele,
    pub data: GI,
    /// If this is a record constructor,
    /// we fill the fields' names here.
    pub fields: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataInfo {
    pub loc: Loc,
    pub name: Ident,
    pub params: Tele,
    /// References to its constructors.
    pub conses: Vec<GI>,
    pub level: Level,
}

/// Declaration.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Monad.Base.html#Function).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decl {
    /// Datatypes.
    Data(DataInfo),
    /// Coinductive records.
    Codata(CodataInfo),
    Cons(ConsInfo),
    Proj {
        loc: Loc,
        name: Ident,
        codata: GI,
        ty: Term,
    },
    /// Function definitions.
    Func {
        loc: Loc,
        name: Ident,
        signature: Term,
        clauses: Vec<Clause>,
    },
}

impl Decl {
    pub fn def_name(&self) -> &Ident {
        match self {
            Decl::Proj { name, .. } | Decl::Func { name, .. } => name,
            Decl::Data(i) => &i.name,
            Decl::Cons(i) => &i.name,
            Decl::Codata(i) => &i.name,
        }
    }
}

impl ToLoc for Decl {
    fn loc(&self) -> Loc {
        match self {
            Decl::Proj { loc, .. } | Decl::Func { loc, .. } => *loc,
            Decl::Data(i) => i.loc(),
            Decl::Cons(i) => i.loc(),
            Decl::Codata(i) => i.loc(),
        }
    }
}

/// Function clauses.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Internal.html#Clause).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clause {
    /// $\Delta$. The types of the pattern variables in dependency order.
    pat_tele: Tele,
    /// $\Delta \vdash ps$. The de Bruijn indices refer to $\Delta$.
    patterns: Vec<Pat>,
    /// `Some(v)` if $\Delta \vdash v$, while `None` if the patterns are absurd.
    body: Option<Term>,
    // TODO: case-trees.
}
