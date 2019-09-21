use std::rc::Rc;

use voile_util::uid::DBI;

use super::{Pat, Term};

pub use self::apply::*;
pub use self::redex::*;
use std::ops::Deref;

/// Substitution type.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Internal.html#Substitution%27).
pub enum PrimSubst<T> {
    /// The identity substitution.
    /// $$
    /// \Gamma \vdash \text{IdS} : \Gamma
    /// $$
    IdS,
    /// The "add one more" substitution, or "substitution extension".
    /// $$
    /// \cfrac{\Gamma \vdash u : A \rho \quad \Gamma \vdash \rho : \Delta}
    /// {\Gamma \vdash \text{Cons}(u, \rho) : \Delta, A}
    /// $$
    Cons(T, Rc<Self>),
    /// Strengthening substitution.
    /// Apply this to a term which does not contain variable 0
    /// to lower all de Bruijn indices by one.
    /// $$
    /// \cfrac{\Gamma \vdash \rho : \Delta}
    /// {\Gamma \vdash \text{Succ} \rho : \Delta, A}
    /// $$
    Succ(Rc<Self>),
    /// Weakning substitution, lifts to an extended context.
    /// $$
    /// \cfrac{\Gamma \vdash \rho : \Delta}
    /// {\Gamma, \Phi \vdash \text{Weak} \mid \Phi \mid \rho : \Delta}
    /// $$
    Weak(usize, Rc<Self>),
}

impl<T> Default for PrimSubst<T> {
    fn default() -> Self {
        PrimSubst::IdS
    }
}

impl<T> PrimSubst<T> {
    /// If lookup failed, return the DBI.
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#lookupS).
    pub fn lookup(&self, dbi: DBI) -> Result<&T, DBI> {
        use PrimSubst::*;
        match self {
            IdS => Err(dbi),
            Cons(o, rest) => match dbi.nat() {
                None => Ok(o),
                Some(dbi) => rest.lookup(dbi),
            },
            Succ(rest) => rest.lookup(dbi.pred()),
            Weak(i, rest) => match &**rest {
                IdS => Err(dbi + *i),
                // TODO: apply_subst to this lookup result.
                rho => rho.lookup(DBI(*i)),
            },
        }
    }

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#raiseS).
    pub fn raise(by: usize) -> Rc<Self> {
        Self::weaken(Default::default(), by)
    }

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#wkS).
    pub fn weaken(me: Rc<Self>, by: usize) -> Rc<Self> {
        use PrimSubst::*;
        match (by, me) {
            (0, rho) => rho,
            (n, rho) => match &*rho {
                Weak(m, rho) => Rc::new(Weak(n + *m, rho.clone())),
                // EmptyS(err) => EmptyS(err)
                _ => Rc::new(Weak(n, rho)),
            },
        }
    }
}

pub type Subst = PrimSubst<Term>;
pub type PatSubst = PrimSubst<Pat>;

/// Eliminate something with something else.
mod apply;
/// Reduction function (red-ex stands for **red**ducible **ex**pression).
mod redex;
