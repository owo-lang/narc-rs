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
    pub val: Option<T>,
}

impl<T> Bind<T> {
    pub fn new(licit: Plicit, name: UID, ty: T, val: Option<T>) -> Self {
        Self {
            licit,
            name,
            ty,
            val,
        }
    }

    pub fn is_implicit(&self) -> bool {
        self.licit == Plicit::Im
    }

    pub fn into_implicit(mut self) -> Self {
        self.licit = Plicit::Im;
        self
    }

    pub fn boxed(self) -> Bind<Box<T>> {
        Bind::boxing(self.licit, self.name, self.ty, self.val)
    }

    pub fn map_term<R>(self, f: impl FnOnce(T) -> R, g: impl FnOnce(T) -> Option<R>) -> Bind<R> {
        Bind::new(self.licit, self.name, f(self.ty), self.val.and_then(g))
    }
}

impl<T> Bind<Box<T>> {
    pub fn boxing(licit: Plicit, name: UID, ty: T, val: Option<T>) -> Self {
        Self::new(licit, name, Box::new(ty), val.map(Box::new))
    }

    pub fn unboxed(self) -> Bind<T> {
        self.map_term(|t| *t, |v| Some(*v))
    }
}
