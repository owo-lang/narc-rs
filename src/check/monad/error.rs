use std::fmt::{Display, Error as FmtError, Formatter};

use voile_util::level::Level;
use voile_util::loc::{Loc, ToLoc};
use voile_util::meta::MI;

use crate::syntax::abs::Abs;
use crate::syntax::core::{Elim, Term};

/// Type-Checking Error.
pub enum TCE {
    Textual(String),
    Wrapped(Box<Self>, Loc),

    // === *Mismatch === //
    // Existing: second. Bad thing: first.
    LevelMismatch(Loc, Level, Level),
    FieldCodataMismatch(Loc, String, String),

    // === Not* === //
    NotHead(Abs),
    NotPi(Term, Loc),
    NotProj(Abs),
    /// A projection is not a term.
    NotTerm(String),

    // === Split* === //
    SplitOnNonVar(Box<Term>, Box<Term>),

    // === Meta* === //
    MetaRecursion(MI),

    // === Different* === //
    DifferentTerm(Box<Term>, Box<Term>),
    DifferentElim(Box<Elim>, Box<Elim>),
    DifferentName(String, String),
}

impl TCE {
    pub fn wrap(self, info: Loc) -> Self {
        TCE::Wrapped(Box::new(self), info)
    }

    fn boxing_two<A, B>(a: A, b: B, f: impl FnOnce(Box<A>, Box<B>) -> Self) -> Self {
        f(Box::new(a), Box::new(b))
    }

    pub fn different_term(a: Term, b: Term) -> Self {
        Self::boxing_two(a, b, TCE::DifferentTerm)
    }

    pub fn different_elim(a: Elim, b: Elim) -> Self {
        Self::boxing_two(a, b, TCE::DifferentElim)
    }

    pub fn split_on_non_var(a: Term, b: Term) -> Self {
        Self::boxing_two(a, b, TCE::SplitOnNonVar)
    }
}

impl Display for TCE {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            TCE::Textual(text) => f.write_str(text),
            TCE::Wrapped(inner, info) => {
                write!(f, "{}\nWhen checking the expression at: {}.", inner, info)
            }
            TCE::LevelMismatch(expr, expected_to_be_small, big) => write!(
                f,
                "Expression `{}` has level {}, which is not smaller than {}.",
                expr, expected_to_be_small, big
            ),
            TCE::FieldCodataMismatch(loc, field, codata) => write!(
                f,
                "Codata `{}` does not have field `{}` (at {}).",
                codata, field, loc
            ),
            // TODO: Display
            TCE::NotHead(abs) => write!(f, "`{:?}` is not a head (at {}).", abs, abs.loc()),
            TCE::NotPi(term, loc) => write!(f, "`{}` is not a pi type (at {}).", term, loc),
            TCE::NotProj(abs) => write!(f, "`{:?}` is not a projection (at {}).", abs, abs.loc()),
            TCE::NotTerm(proj) => write!(f, "Cannot project `{}` on a datatype.", proj),
            TCE::SplitOnNonVar(term, ty) => {
                write!(f, "Splitting on non variable `{}` (of type `{}`)", term, ty)
            }
            TCE::MetaRecursion(mi) => {
                write!(f, "Trying to solve a recursive meta of index {}.", mi)
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
