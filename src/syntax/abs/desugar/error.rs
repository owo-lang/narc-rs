use std::fmt::{Display, Error as FmtError, Formatter};

use voile_util::loc::Ident;

#[derive(Debug, Clone)]
pub enum DesugarErr {
    UnresolvedReference(Ident),

    // === Not* === //
    NotDefn(Ident),
    NotCons(Ident),
}

impl Display for DesugarErr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        use DesugarErr::*;
        match self {
            UnresolvedReference(i) => write!(f, "Unresolved reference: `{}` at {}.", i.text, i.loc),
            NotDefn(i) => write!(f, "`{}` is not a definition (at {}).", i.text, i.loc),
            NotCons(i) => write!(f, "`{}` is not a constructor (at {}).", i.text, i.loc),
        }
    }
}
