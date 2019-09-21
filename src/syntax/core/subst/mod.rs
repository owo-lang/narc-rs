use std::rc::Rc;

use voile_util::uid::DBI;

use super::{Pat, Term};

pub use self::apply::*;
pub use self::redex::*;

/// Substitution type.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Internal.html#Substitution%27).
pub enum PrimSubst<T> {
    /// The identity substitution.
    /// $$
    /// \Gamma \vdash \text{IdS} : \Gamma
    /// $$
    IdS,
    /// The "add one more" substitution.
    /// $$
    /// \cfrac{\Gamma \vdash u : A \rho \quad \Gamma \vdash \rho : \Delta}
    /// {\Gamma \vdash \text{Cons}(u, \rho) : \Delta, A}
    /// $$
    Cons(T, Rc<Self>),
}

impl<T> PrimSubst<T> {
    /// If lookup failed, return the DBI.
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#lookupS).
    pub fn lookup(&self, dbi: DBI) -> Result<&T, DBI> {
        use PrimSubst::*;
        match self {
            IdS => Err(dbi),
            Cons(o, _) if dbi == DBI(0) => Ok(o),
            Cons(..) if dbi < DBI(0) => unreachable!(),
            Cons(_, rest) => rest.lookup(DBI(dbi.0 - 1)),
        }
    }
}

pub type Subst = PrimSubst<Term>;
pub type PatSubst = PrimSubst<Pat>;

/// Eliminate something with something else.
mod apply;
/// Reduction function (red-ex stands for **red**ducible **ex**pression).
mod redex;
