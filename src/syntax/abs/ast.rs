use voile_util::loc::{Ident, Loc, ToLoc};
use voile_util::meta::MI;
use voile_util::uid::{GI, UID};

use crate::syntax::pat::Copat;
use voile_util::level::Level;

/// The abstract syntax.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Abs {
    Def(Ident, GI),
    Var(Ident, UID),
    Meta(Ident, MI),
    App(Loc, Box<Self>, Box<Self>),
    Pi(Loc, AbsTele, Box<Self>),
    Type(Ident, Level),
    Cons(Ident, GI),
    Proj(Ident, GI),
    TODO, // TODO
}

/// Application's internal view.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.Views.html#AppView).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppView {
    pub fun: Abs,
    pub args: Vec<(Loc, Abs)>,
}

impl AppView {
    pub fn new(fun: Abs, args: Vec<(Loc, Abs)>) -> Self {
        Self { fun, args }
    }

    pub fn into_abs(self) -> Abs {
        self.args
            .into_iter()
            .fold(self.fun, |f, (loc, arg)| Abs::app(loc, f, arg))
    }
}

impl Abs {
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.Views.html#appView).
    pub fn into_app_view(self) -> AppView {
        match self {
            Abs::App(loc, f, arg) => {
                let mut view = f.into_app_view();
                view.args.push((loc, *arg));
                view
            }
            e => AppView::new(e, vec![]),
        }
    }

    pub fn app(loc: Loc, f: Self, arg: Self) -> Self {
        Abs::App(loc, Box::new(f), Box::new(arg))
    }
}

impl ToLoc for Abs {
    fn loc(&self) -> Loc {
        use Abs::*;
        match self {
            Proj(ident, ..) | Cons(ident, ..) | Type(ident, ..) | Def(ident, ..)
            | Var(ident, ..) | Meta(ident, ..) => ident.loc,
            App(loc, ..) | Pi(loc, ..) => *loc,
            TODO => unreachable!(),
        }
    }
}

/// Telescopes in the abstract syntax.
pub type AbsTele = Vec<Abs>;

/// Patterns in the abstract syntax.
pub type AbsPat = Copat<UID, Abs>;
