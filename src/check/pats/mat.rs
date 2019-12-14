use std::collections::HashMap;

use voile_util::uid::DBI;

use crate::syntax::core::Term;

/// `Simplification` in
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Monad.Base.html#Simplification).
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Simpl {
    Yes,
    No,
}

impl From<bool> for Simpl {
    fn from(b: bool) -> Self {
        if b {
            Simpl::Yes
        } else {
            Simpl::No
        }
    }
}

impl Into<bool> for Simpl {
    fn into(self) -> bool {
        match self {
            Simpl::Yes => true,
            Simpl::No => false,
        }
    }
}

#[derive(Debug, Clone)]
enum Match {
    Yes(Simpl, HashMap<DBI, Term>),
    No,
}
