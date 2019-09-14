use voile_util::level::Level;
use voile_util::loc::Labelled;

use super::{Tele, Term};

/// Declaration.
/// https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Monad.Base.html#Function
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decl {
    /// Datatypes. The parameters, the constructors, and the level.
    Data(Tele, Vec<Constructor>, Level),
    /// Coinductive records.
    /// The self reference, the name, the fields and the level.
    Codata(String, String, Vec<Field>, Level),
}

/// Constructors.
/// This definition follows the paper version instead
/// of the one in Agda's implementation.
pub type Constructor = Labelled<Tele>;

/// Fields.
/// This definition follows the paper version instead
/// of the one in Agda's implementation.
pub type Field = Labelled<Term>;
