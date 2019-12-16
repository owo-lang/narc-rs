use voile_util::{
    loc::Ident,
    tags::Plicit,
    uid::{GI, UID},
};

/// Inductive or coinductive?
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum Ductive {
    In,
    Coin,
}

/// Parameter information -- with type and visibility.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bind<T> {
    pub licit: Plicit,
    pub name: UID,
    pub ty: T,
}

/// Let binding.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Let<T> {
    pub bind: Bind<T>,
    pub val: T,
}

impl<T> Bind<T> {
    pub fn new(licit: Plicit, name: UID, ty: T) -> Self {
        Self { licit, name, ty }
    }

    pub fn is_implicit(&self) -> bool {
        self.licit == Plicit::Im
    }

    pub fn into_implicit(mut self) -> Self {
        self.licit = Plicit::Im;
        self
    }

    pub fn boxed(self) -> Bind<Box<T>> {
        Bind::boxing(self.licit, self.name, self.ty)
    }

    pub fn map_term<R>(self, f: impl FnOnce(T) -> R) -> Bind<R> {
        Bind::new(self.licit, self.name, f(self.ty))
    }
}

impl<T> Bind<Box<T>> {
    pub fn boxing(licit: Plicit, name: UID, term: T) -> Self {
        Self::new(licit, name, Box::new(term))
    }

    pub fn unboxed(self) -> Bind<T> {
        self.map_term(|t| *t)
    }
}

impl<T> Let<T> {
    pub fn new(bind: Bind<T>, val: T) -> Self {
        Self { bind, val }
    }
}

/// Constructor information.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Internal.html#ConHead).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConHead {
    /// Constructor name.
    pub name: Ident,
    /// Index of the constructor.
    pub cons_ix: GI,
    /// Records might be coinductive.
    pub ductive: Ductive,
    /// Field names.
    /// This allows us to project fields from a record without the `TCS`.
    pub fields: Vec<String>,
}

impl ConHead {
    pub fn pseudo(name: Ident) -> Self {
        Self::new(name, Default::default(), Ductive::In, vec![])
    }

    pub fn new(name: Ident, ix: GI, ductive: Ductive, fields: Vec<String>) -> Self {
        Self {
            name,
            cons_ix: ix,
            ductive,
            fields,
        }
    }
}
