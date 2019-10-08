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
    App(Box<Self>, Box<Self>),
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
        self.args
            .into_iter()
            .fold(self.fun, |f, arg| Abs::app(f, arg))
    }
}

impl Abs {
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.Views.html#appView).
    pub fn into_app_view(self) -> AppView {
        match self {
            Abs::App(f, arg) => {
                let mut view = f.into_app_view();
                view.args.push(*arg);
                view
            }
            e => AppView::new(e, vec![]),
        }
    }

    pub fn app(f: Self, arg: Self) -> Self {
        Abs::App(Box::new(f), Box::new(arg))
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
            App(f, a) => f.loc() + a.loc(),
        }
    }
}

/// Name binding.
pub type Bind<T = Abs> = common::Bind<T>;

/// Telescopes in the abstract syntax.
pub type AbsTele = Vec<Bind>;

/// Patterns in the abstract syntax.
pub type AbsPat = Copat<UID, Abs>;
