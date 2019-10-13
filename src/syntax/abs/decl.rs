use voile_util::level::Level;
use voile_util::loc::{Ident, Loc, ToLoc};
use voile_util::uid::GI;

use crate::syntax::core::Term;

use super::{Abs, AbsCopat, AbsTele};

/// Declaration.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Abstract.html#t:Declaration).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AbsDecl {
    /// Datatypes.
    Data {
        source: Loc,
        name: Ident,
        level: Level,
        tele: AbsTele,
        conses: Vec<GI>,
    },
    /// Datatype constructors.
    Cons {
        source: Loc,
        name: Ident,
        tele: AbsTele,
        /// Corresponding datatype's index.
        data_ix: GI,
    },
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
    Clause { source: Loc, info: AbsClause },
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

    pub fn cons(source: Loc, name: Ident, tele: AbsTele, data_index: GI) -> Self {
        AbsDecl::Cons {
            source,
            name,
            tele,
            data_ix: data_index,
        }
    }

    pub fn field(source: Loc, name: Ident, proj_ty: Abs, codata_index: GI) -> Self {
        AbsDecl::Proj {
            source,
            name,
            ty: proj_ty,
            codata_ix: codata_index,
        }
    }

    pub fn data(source: Loc, name: Ident, level: Level, tele: AbsTele, conses: Vec<GI>) -> Self {
        AbsDecl::Data {
            source,
            name,
            level,
            tele,
            conses,
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

    pub fn clause(source: Loc, info: AbsClause) -> Self {
        AbsDecl::Clause { source, info }
    }

    pub fn decl_name(&self) -> &Ident {
        use AbsDecl::*;
        match self {
            Data { name, .. }
            | Cons { name, .. }
            | Proj { name, .. }
            | Defn { name, .. }
            | Codata { name, .. } => name,
            Clause { info, .. } => &info.name,
        }
    }
}

impl ToLoc for AbsDecl {
    fn loc(&self) -> Loc {
        use AbsDecl::*;
        match self {
            Data { source, .. }
            | Cons { source, .. }
            | Proj { source, .. }
            | Defn { source, .. }
            | Clause { source, .. }
            | Codata { source, .. } => *source,
        }
    }
}

/// A user pattern and a core term that they should equal
/// after splitting is complete.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.html#ProblemEq).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProblemEq {
    in_pat: AbsCopat,
    inst: Term,
    ty: Term,
}

/// Clause information in abstract syntax.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.html#Clause%27).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AbsClause {
    /// Name of the function we're adding clause to.
    pub name: Ident,
    /// Lhs.
    pub patterns: Vec<AbsCopat>,
    /// Index of the type signature definition.
    pub definition: GI,
    /// Rhs.
    pub body: Abs,
}
