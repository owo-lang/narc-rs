use voile_util::level::Level;
use voile_util::loc::{Loc, ToLoc};
use voile_util::uid::GI;

use crate::syntax::core::Pat;

use super::{Tele, Term};

/// Declaration.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Monad.Base.html#Function).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decl {
    /// Datatypes.
    Data {
        loc: Loc,
        name: String,
        params: Tele,
        /// References to its constructors.
        conses: Vec<GI>,
        level: Level,
    },
    /// Coinductive records.
    Codata {
        loc: Loc,
        self_ref: String,
        name: String,
        params: Tele,
        /// References to its projections (fields).
        fields: Vec<GI>,
        level: Level,
    },
    Cons {
        loc: Loc,
        name: String,
        params: Tele,
        data: GI,
        /// If this is a record constructor,
        /// we fill the fields' names here.
        fields: Option<Vec<String>>,
    },
    Proj {
        loc: Loc,
        name: String,
        codata: GI,
        ty: Term,
    },
    /// Function definitions.
    Func {
        loc: Loc,
        name: String,
        signature: Term,
        clauses: Vec<Clause>,
    },
}

impl ToLoc for Decl {
    fn loc(&self) -> Loc {
        match self {
            Decl::Data { loc, .. }
            | Decl::Codata { loc, .. }
            | Decl::Cons { loc, .. }
            | Decl::Proj { loc, .. }
            | Decl::Func { loc, .. } => *loc,
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
