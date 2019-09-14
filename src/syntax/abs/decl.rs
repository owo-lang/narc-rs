use voile_util::level::Level;
use voile_util::loc::{Ident, Labelled, Loc};

use super::AbsTele;

/// Declaration.
/// https://hackage.haskell.org/package/Agda-2.6.0.1/docs/Agda-Syntax-Abstract.html#t:Declaration
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decl {
    /// Datatypes. Location, the name, the constructors, and the level.
    Data(Loc, Ident, Vec<Constructor>, Level),
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
