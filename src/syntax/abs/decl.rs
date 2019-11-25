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
    Proj {
        source: Loc,
        name: Ident,
        ty: Abs,
        /// Corresponding coinductive record's index.
        codata_ix: GI,
    },
    /// Function signature definition.
    Defn { source: Loc, name: Ident, ty: Abs },
    /// Pattern matching clause.
    Clause(AbsClause),
    /// Coinductive records.
    Codata {
        source: Loc,
        self_ref: Option<Ident>,
        name: Ident,
        fields: Vec<GI>,
        level: Level,
        tele: AbsTele,
    },
}

impl AbsDecl {
    pub fn defn(source: Loc, name: Ident, ty: Abs) -> Self {
        AbsDecl::Defn { source, name, ty }
    }

    pub fn field(source: Loc, name: Ident, proj_ty: Abs, codata_index: GI) -> Self {
        AbsDecl::Proj {
            source,
            name,
            ty: proj_ty,
            codata_ix: codata_index,
        }
    }

    pub fn codata(
        source: Loc,
        name: Ident,
        me: Option<Ident>,
        level: Level,
        tele: AbsTele,
        fields: Vec<GI>,
    ) -> Self {
        AbsDecl::Codata {
            source,
            name,
            self_ref: me,
            level,
            tele,
            fields,
        }
    }

    pub fn decl_name(&self) -> &Ident {
        use AbsDecl::*;
        match self {
            Proj { name, .. } | Defn { name, .. } | Codata { name, .. } => name,
            Clause(info) => &info.name,
            Data(info) => &info.name,
            Cons(info) => &info.name,
        }
    }
}

impl ToLoc for AbsDataInfo {
    fn loc(&self) -> Loc {
        self.source
    }
}

impl ToLoc for AbsConsInfo {
    fn loc(&self) -> Loc {
        self.source
    }
}

impl ToLoc for AbsClause {
    fn loc(&self) -> Loc {
        self.source
    }
}

impl ToLoc for AbsDecl {
    fn loc(&self) -> Loc {
        use AbsDecl::*;
        match self {
            Proj { source, .. } | Defn { source, .. } | Codata { source, .. } => *source,
            Data(i) => i.loc(),
            Cons(i) => i.loc(),
            Clause(i) => i.loc(),
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
