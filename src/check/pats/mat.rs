use std::collections::HashMap;
use std::ops::Add;

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

impl Default for Simpl {
    fn default() -> Self {
        Simpl::No
    }
}

impl Add for Simpl {
    type Output = Simpl;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Simpl::Yes => Simpl::Yes,
            Simpl::No => rhs,
        }
    }
}

#[derive(Debug, Clone)]
enum Match {
    Yes(Simpl, HashMap<DBI, Term>),
    No,
}

impl Default for Match {
    fn default() -> Self {
        Match::Yes(Default::default(), Default::default())
    }
}

impl Add for Match {
    type Output = Match;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Match::No, o) => o,
            (o, Match::No) => o,
            (Match::Yes(s0, mut m0), Match::Yes(s1, m1)) => {
                m0.extend(m1.into_iter());
                Match::Yes(s0 + s1, m0)
            }
        }
    }
}
