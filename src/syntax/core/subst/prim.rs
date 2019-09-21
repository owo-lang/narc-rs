use std::rc::Rc;

use either::Either;
use voile_util::uid::DBI;

use super::super::{Pat, Term};
use super::*;

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
    /// Weakening substitution, lifts to an extended context.
    /// $$
    /// \cfrac{\Gamma \vdash \rho : \Delta}
    /// {\Gamma, \Psi \vdash \text{Weak}_\Psi \rho : \Delta}
    /// $$
    Weak(usize, Rc<Self>),
    /// Lifting substitution. Use this to go under a binder.
    /// $\text{Lift}\_1 \rho := \text{Cons}(\texttt{Term::form\\\_dbi(0)}, \text{Weak}\_1 \rho)$.
    /// $$
    /// \cfrac{\Gamma \vdash \rho : \Delta}
    /// {\Gamma, \Psi \rho \vdash \text{Lift}_\Psi \rho : \Delta, \Psi}
    /// $$
    Lift(usize, Rc<Self>),
}

impl<T> Default for PrimSubst<T> {
    fn default() -> Self {
        PrimSubst::IdS
    }
}

impl PrimSubst<Term> {
    /// If lookup failed, return the DBI.
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#lookupS).
    pub fn lookup_impl(&self, dbi: DBI) -> Either<&Term, Term> {
        use Either::*;
        use PrimSubst::*;
        match self {
            IdS => Right(Term::from_dbi(dbi)),
            Cons(o, rest) => match dbi.nat() {
                None => Left(o),
                Some(dbi) => rest.lookup_impl(dbi),
            },
            Succ(rest) => rest.lookup_impl(dbi.pred()),
            Weak(i, rest) => match &**rest {
                IdS => Right(Term::from_dbi(dbi + *i)),
                rho => Right(rho.lookup(DBI(*i)).reduce_dbi(&*Self::raise(*i))),
            },
            Lift(n, rest) => {
                if dbi < DBI(*n) {
                    Right(Term::from_dbi(dbi))
                } else {
                    Right(Self::raise_term(*n, rest.lookup(DBI(dbi.0 - *n))))
                }
            }
        }
    }

    pub fn lookup(&self, dbi: DBI) -> Term {
        self.lookup_impl(dbi).map_left(Clone::clone).into_inner()
    }

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#raise).
    pub fn raise_term(k: usize, term: Term) -> Term {
        Self::raise_from(0, k, term)
    }

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#raiseFrom).
    pub fn raise_from(n: usize, k: usize, term: Term) -> Term {
        term.reduce_dbi(&Self::lift(Self::raise(k), n))
    }
}

impl<T> PrimSubst<T> {
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#raiseS).
    pub fn raise(by: usize) -> Rc<Self> {
        Self::weaken(Default::default(), by)
    }

    /// Lift a substitution under k binders.
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#liftS).
    pub fn lift(me: Rc<Self>, by: usize) -> Rc<Self> {
        use PrimSubst::*;
        match (by, me) {
            (0, rho) => rho,
            (k, rho) => match &*rho {
                IdS => Default::default(),
                Lift(n, rho) => Rc::new(Lift(n + k, rho.clone())),
                _ => Rc::new(Lift(k, rho)),
            },
        }
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
