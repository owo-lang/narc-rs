use voile_util::level::Level;
use voile_util::loc::{Ident, Labelled, Loc};

use super::{Abs, AbsTele};
use crate::syntax::abs::AbsPat;

/// Declaration.
/// https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Abstract.html#t:Declaration
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AbsDecl {
    /// Datatypes. Location, the name, the constructors, and the level.
    Data(Loc, Ident, Level),
    /// Datatype constructors. Location, the name, the parameters,
    /// the corresponding datatype's name, the arguments applied to the datatype.
    Cons(Loc, Ident, AbsTele, Ident, AbsTele),
    /// Coinductive record projections. Location, the name, the parameters,
    /// the corresponding coinductive record's name, the arguments applied to the record.
    Proj(Loc, Ident, AbsTele, Ident, AbsTele),
    /// Function signature definition.
    Defn(Loc, Ident, Abs),
    /// Pattern matching clause. The location, the name, the patterns and the body.
    Clause(Loc, Ident, Vec<AbsPat>, Abs),
    /// Coinductive records. Location,
    /// the self reference, the name, the fields and the level.
    Codata(Loc, Ident, Ident, Vec<Field>, Level),
}

/// Constructors.
/// This definition follows the paper version instead
/// of the one in Agda's implementation.
pub type Constructor = Labelled<AbsTele>;

/// Fields.
/// This definition follows the paper version instead
/// of the one in Agda's implementation.
pub type Field = Labelled<AbsTele>;
