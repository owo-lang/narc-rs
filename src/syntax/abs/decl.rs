use voile_util::level::Level;
use voile_util::loc::{Ident, Labelled, Loc, ToLoc};

use crate::syntax::core::Term;

use super::{Abs, AbsCopat, AbsTele};
use voile_util::uid::GI;

/// Declaration.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Abstract.html#t:Declaration).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AbsDecl {
    /// Datatypes.
    Data {
        source: Loc,
        name: Ident,
        level: Level,
    },
    /// Datatype constructors.
    Cons {
        source: Loc,
        name: Ident,
        params: AbsTele,
        /// Corresponding datatype's name.
        data_name: Ident,
        /// Arguments applied to the datatype.
        data_tele: AbsTele,
    },
    /// Coinductive record projections.
    Proj {
        source: Loc,
        name: Ident,
        ty: Abs,
        /// Corresponding coinductive record's name
        codata_name: Ident,
        /// Arguments applied to the record.
        codata_tele: AbsTele,
    },
    /// Function signature definition.
    Defn { source: Loc, name: Ident, ty: Abs },
    /// Pattern matching clause.
    Clause { source: Loc, info: AbsClause },
    /// Coinductive records.
    Codata {
        source: Loc,
        self_ref: Ident,
        name: Ident,
        fields: Vec<Field>,
        level: Level,
    },
}

impl AbsDecl {
    pub fn defn(source: Loc, name: Ident, ty: Abs) -> Self {
        AbsDecl::Defn { source, name, ty }
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
    name: Ident,
    /// Lhs.
    patterns: Vec<AbsCopat>,
    /// Index of the type signature definition.
    definition: GI,
    /// Rhs.
    body: Abs,
}

/// Constructors.
/// This definition follows the paper version instead
/// of the one in Agda's implementation.
pub type Constructor = Labelled<AbsTele>;

/// Fields.
/// This definition follows the paper version instead
/// of the one in Agda's implementation.
pub type Field = Labelled<AbsTele>;
