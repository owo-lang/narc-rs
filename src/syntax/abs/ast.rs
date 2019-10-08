use voile_util::level::Level;
use voile_util::loc::{Ident, Loc, ToLoc};
use voile_util::meta::MI;
use voile_util::uid::{GI, UID};

use crate::syntax::common;
use crate::syntax::pat::Copat;

/// The abstract syntax.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Abs {
    Def(Ident, GI),
    Var(Ident, UID),
    Meta(Ident, MI),
    App(Box<Self>, Vec<Self>),
    Pi(Loc, Bind<Box<Self>>, Box<Self>),
    Type(Ident, Level),
    Cons(Ident, GI),
    Proj(Ident, GI),
}

/// Application's internal view.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.Views.html#AppView).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppView {
    pub fun: Abs,
    pub args: Vec<Abs>,
}

impl AppView {
    pub fn new(fun: Abs, args: Vec<Abs>) -> Self {
        Self { fun, args }
    }

    pub fn into_abs(self) -> Abs {
        Abs::app(self.fun, self.args)
    }
}

impl Abs {
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.Views.html#appView).
    pub fn into_app_view(self) -> AppView {
        match self {
            Abs::App(f, mut arg) => {
                let mut view = f.into_app_view();
                view.args.append(&mut arg);
                view
            }
            e => AppView::new(e, vec![]),
        }
    }

    pub fn simple_app(f: Self, arg: Self) -> Self {
        Self::app(f, vec![arg])
    }

    pub fn app(f: Self, args: Vec<Self>) -> Self {
        Abs::App(Box::new(f), args)
    }
}

impl ToLoc for Abs {
    fn loc(&self) -> Loc {
        use Abs::*;
        match self {
            Proj(ident, ..)
            | Cons(ident, ..)
            | Type(ident, ..)
            | Def(ident, ..)
            | Var(ident, ..)
            | Meta(ident, ..) => ident.loc,
            Pi(loc, ..) => *loc,
            // TODO: improve by making `a` a `Vec1`.
            App(f, a) => f.loc() + a.last().unwrap().loc(),
        }
    }
}

/// Name binding.
pub type Bind<T = Abs> = common::Bind<T>;

/// Telescopes in the abstract syntax.
pub type AbsTele = Vec<Bind>;

/// Patterns in the abstract syntax.
pub type AbsPat = Copat<UID, Abs>;
