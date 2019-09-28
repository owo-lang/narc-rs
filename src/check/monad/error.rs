use crate::syntax::abs::Abs;
use crate::syntax::core::{Elim, Term};
use std::fmt::{Display, Error as FmtError, Formatter};
use voile_util::level::Level;
use voile_util::loc::Loc;

/// Type-Checking Error.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TCE {
    /// Expected the first level to be smaller than second.
    LevelMismatch(Loc, Level, Level),
    Textual(String),
    Wrapped(Box<Self>, Loc),

    // === Not* === //
    NotHead(Abs),
    NotPi(Term, Loc),

    // === Different* === //
    DifferentTerm(Term, Term),
    DifferentElim(Elim, Elim),
    DifferentName(String, String),
}

impl TCE {
    pub fn wrap(self, info: Loc) -> Self {
        TCE::Wrapped(Box::new(self), info)
    }
}

impl Display for TCE {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            TCE::LevelMismatch(expr, expected_to_be_small, big) => write!(
                f,
                "Expression `{}` has level {}, which is not smaller than {}.",
                expr, expected_to_be_small, big
            ),
            TCE::Textual(text) => f.write_str(text),
            TCE::Wrapped(inner, info) => {
                write!(f, "{}\nWhen checking the expression at: {}.", inner, info)
            }
            // TODO: Display
            TCE::NotHead(abs) => write!(f, "`{:?}` is not a head expression.", abs),
            TCE::NotPi(term, loc) => {
                write!(f, "`{}` is not a pi type expression (at {}).", term, loc)
            }
            TCE::DifferentTerm(a, b) => write!(f, "Failed to unify `{}` and `{}`.", a, b),
            TCE::DifferentElim(a, b) => write!(f, "Failed to unify `{}` and `{}`.", a, b),
            TCE::DifferentName(a, b) => write!(
                f,
                "`{}` and `{}` are different (conversion check is not structural in Narc).",
                a, b
            ),
        }
    }
}
