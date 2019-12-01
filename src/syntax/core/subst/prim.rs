use std::rc::Rc;

use voile_util::uid::DBI;

use super::super::{Pat, Term};

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
    Weak(DBI, Rc<Self>),
    /// Lifting substitution. Use this to go under a binder.
    /// $\text{Lift}\_1 \rho := \text{Cons}(\texttt{Term::form\\\_dbi(0)}, \text{Weak}\_1 \rho)$.
    /// $$
    /// \cfrac{\Gamma \vdash \rho : \Delta}
    /// {\Gamma, \Psi \rho \vdash \text{Lift}_\Psi \rho : \Delta, \Psi}
    /// $$
    Lift(DBI, Rc<Self>),
}

impl<T> Default for PrimSubst<T> {
    fn default() -> Self {
        PrimSubst::IdS
    }
}

impl<T> PrimSubst<T> {
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#raiseS).
    pub fn raise(by: DBI) -> Rc<Self> {
        Self::weaken(Default::default(), by)
    }

    /// Lift a substitution under k binders.
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#dropS).
    pub fn drop_by(me: Rc<Self>, drop_by: DBI) -> Rc<Self> {
        use PrimSubst::*;
        match (drop_by, &*me) {
            (DBI(0), _) => me,
            (n, IdS) => Self::raise(n),
            (n, Weak(m, rho)) => Self::weaken(Self::drop_by(rho.clone(), n - 1), *m),
            (n, Cons(_, rho)) | (n, Succ(rho)) => Self::drop_by(rho.clone(), n - 1),
            // n, EmptyS(err) => absurd(err)
            (n, Lift(DBI(0), _rho)) => unreachable!(&format!("n = {:?}", n)),
            (n, Lift(m, rho)) => Self::weaken(
                Self::drop_by(Self::lift_by(rho.clone(), *m - 1), n - 1),
                DBI(1),
            ),
        }
    }

    /// Lift a substitution under k binders.
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#liftS).
    pub fn lift_by(me: Rc<Self>, lift_by: DBI) -> Rc<Self> {
        use PrimSubst::*;
        match (lift_by, &*me) {
            (DBI(0), _) => me,
            (_, IdS) => Default::default(),
            (k, Lift(n, rho)) => Rc::new(Lift(*n + k, rho.clone())),
            (k, _) => Rc::new(Lift(k, me)),
        }
    }

    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.Class.html#wkS).
    pub fn weaken(me: Rc<Self>, weaken_by: DBI) -> Rc<Self> {
        use PrimSubst::*;
        match (weaken_by, &*me) {
            (DBI(0), _) => me,
            (n, Weak(m, rho)) => Rc::new(Weak(n + *m, rho.clone())),
            // n, EmptyS(err) => EmptyS(err)
            (n, _) => Rc::new(Weak(n, me)),
        }
    }

    pub fn one(t: T) -> Self {
        PrimSubst::Cons(t, Default::default())
    }
}

pub type Subst = PrimSubst<Term>;
pub type PatSubst = PrimSubst<Pat>;
