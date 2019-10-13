use voile_util::level::Level;
use voile_util::loc::{Ident, Loc, ToLoc};
use voile_util::meta::MI;
use voile_util::uid::{GI, UID};
use voile_util::vec1::Vec1;

use crate::syntax::common;
use crate::syntax::pat::*;

/// The abstract syntax.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Abs {
    Def(Ident, GI),
    Var(Ident, UID),
    Meta(Ident, MI),
    App(Box<Self>, Box<Vec1<Self>>),
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

    pub fn into_abs(mut self) -> Abs {
        if self.args.is_empty() {
            self.fun
        } else {
            let head = self.args.remove(0);
            Abs::app(self.fun, Vec1::new(head, self.args))
        }
    }
}

impl Abs {
    /// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.Syntax.Abstract.Views.html#appView).
    pub fn into_app_view(self) -> AppView {
        match self {
            Abs::App(f, arg) => {
                let mut view = f.into_app_view();
                arg.append_self_into(&mut view.args);
                view
            }
            e => AppView::new(e, vec![]),
        }
    }

    pub fn simple_app(f: Self, arg: Self) -> Self {
        Self::app(f, From::from(arg))
    }

    pub fn app(f: Self, args: Vec1<Self>) -> Self {
        Abs::App(Box::new(f), Box::new(args))
    }

    pub fn universe(id: Ident) -> Self {
        Abs::universe_at(id, Default::default())
    }

    pub fn meta(id: Ident, mi: MI) -> Self {
        Abs::Meta(id, mi)
    }

    pub fn universe_at(id: Ident, level: Level) -> Self {
        Abs::Type(id, level)
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
            App(f, a) => f.loc() + a.last().loc(),
        }
    }
}

/// Name binding.
pub type Bind<T = Abs> = common::Bind<T>;

/// Telescopes in the abstract syntax.
pub type AbsTele = Vec<Bind>;

/// Patterns in the abstract syntax.
pub type AbsCopat = Copat<UID, Abs>;
pub type AbsPat = Pat<UID, Abs>;
