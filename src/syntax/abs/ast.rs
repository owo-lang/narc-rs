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
    Cons(Ident),
    Proj(Ident),
    TODO, // TODO
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
