use voile_util::level::Level;
use voile_util::loc::{Ident, Labelled, Loc, ToLoc};

use super::{Abs, AbsTele};
use crate::syntax::abs::AbsPat;

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
    Clause {
        source: Loc,
        name: Ident,
        patterns: Vec<AbsPat>,
        body: Abs,
    },
    /// Coinductive records.
    Codata {
        source: Loc,
        self_ref: Ident,
        name: Ident,
        fields: Vec<Field>,
        level: Level,
    },
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

/// Constructors.
/// This definition follows the paper version instead
/// of the one in Agda's implementation.
pub type Constructor = Labelled<AbsTele>;

/// Fields.
/// This definition follows the paper version instead
/// of the one in Agda's implementation.
pub type Field = Labelled<AbsTele>;
