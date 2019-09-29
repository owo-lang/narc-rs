use voile_util::tags::Plicit;
use voile_util::uid::UID;

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

impl<T> Bind<T> {
    pub fn new(licit: Plicit, name: UID, ty: T) -> Self {
        Self { licit, name, ty }
    }

    pub fn into_implicit(mut self) -> Self {
        self.licit = Plicit::Im;
        self
    }

    pub fn map_term<R>(self, f: impl FnOnce(T) -> R) -> Bind<R> {
        Bind::new(self.licit, self.name, f(self.ty))
    }
}

impl<T> Bind<Box<T>> {
    pub fn boxed(licit: Plicit, name: UID, term: T) -> Self {
        Self::new(licit, name, Box::new(term))
    }
}
