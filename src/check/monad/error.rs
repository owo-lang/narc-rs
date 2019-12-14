use std::fmt::{Display, Error as FmtError, Formatter};

use voile_util::level::Level;
use voile_util::loc::{Ident, Loc, ToLoc};
use voile_util::meta::MI;

use crate::syntax::abs::Abs;
use crate::syntax::core::{Elim, Term};

/// Type-Checking Error.
pub enum TCE {
    Textual(String),
    Wrapped(Box<Self>, Loc),

    // === Not* === //
    NotHead(Abs),
    NotPi(Term, Loc),
    NotProj(Abs),
    /// A projection is not a term.
    NotTerm(String),

    // === Split* === //
    SplitOnNonVar(Box<Term>, Box<Term>),

    // === Cant* === //
    CantFindPattern(Ident),
    CantSimplify(Ident),

    // === Meta* === //
    MetaRecursion(MI),
    MetaUnsolved(MI),

    // === Different* === //
    DifferentLevel(Loc, Level, Level),
    DifferentFieldCodata(Loc, String, String),
    DifferentTerm(Box<Term>, Box<Term>),
    DifferentElim(Box<Elim>, Box<Elim>),
    DifferentName(Ident, Ident),
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
            TCE::NotHead(abs) => write!(f, "`{}` is not a head (at {}).", abs, abs.loc()),
            TCE::NotPi(term, loc) => write!(f, "`{}` is not a pi type (at {}).", term, loc),
            TCE::NotProj(abs) => write!(f, "`{:?}` is not a projection (at {}).", abs, abs.loc()),
            TCE::NotTerm(proj) => write!(f, "Cannot project `{}` on a datatype.", proj),
            TCE::SplitOnNonVar(term, ty) => {
                write!(f, "Splitting on non variable `{}` (of type `{}`)", term, ty)
            }
            TCE::CantFindPattern(call) => write!(f, "Didn't find a matchable pattern for `{}` (at {})", call.text, call.loc),
            TCE::CantSimplify(call) => write!(f, "Can't simplify a call to `{}` (at {})", call.text, call.loc),
            TCE::MetaRecursion(mi) => write!(f, "Trying to solve a recursive meta of index {}.", mi),
            TCE::MetaUnsolved(mi) => write!(f, "Unsolved meta of index {}.", mi),
            TCE::DifferentLevel(expr, expected_to_be_small, big) => write!(
                f,
                "Expression `{}` has level {}, which is not smaller than {}.",
                expr, expected_to_be_small, big
            ),
            TCE::DifferentFieldCodata(loc, field, codata) => write!(
                f,
                "Codata `{}` does not have field `{}` (at {}).",
                codata, field, loc
            ),
            TCE::DifferentTerm(a, b) => write!(f, "Failed to unify `{}` and `{}`.", a, b),
            TCE::DifferentElim(a, b) => write!(f, "Failed to unify `{}` and `{}`.", a, b),
            TCE::DifferentName(a, b) => write!(
                f,
                "`{}` (at {}) and `{}` (at {}) are different (conversion check is not structural in Narc).",
                a.text, a.loc, b.text, b.loc
            ),
        }
    }
}
