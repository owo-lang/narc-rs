use std::fmt::{Display, Error, Formatter};

use voile_util::tags::{Plicit, VarRec};
use Plicit::{Ex as Explicit, Im as Implicit};

use crate::syntax::{common::ConHead, core::Bind};

use super::{Closure, Elim, Term, TermInfo, Val};

impl Display for Elim {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Elim::App(app) => app.fmt(f),
            Elim::Proj(field) => write!(f, ".{}", field),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Term::Whnf(v) => v.fmt(f),
            Term::Redex(_, ident, args) => pretty_application(f, &ident.text, args),
        }
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use Val::*;
        match self {
            Meta(mi, a) => {
                f.write_str("?")?;
                pretty_application(f, mi, a)
            }
            Var(fun, a) => pretty_application(f, fun, a),
            Type(l) => write!(f, "set{}", l),
            Pi(Bind { licit, ty, .. }, clos) => match licit {
                Explicit => write!(f, "({} -> {})", ty, clos),
                Implicit => write!(f, "({{{}}} -> {})", ty, clos),
            },
            Cons(name, a) => pretty_application(f, name, a),
            Data(kind, ix, args) => {
                f.write_str(match kind {
                    VarRec::Variant => "data",
                    VarRec::Record => "codata",
                })?;
                pretty_application(f, ix, args)
            }
            Axiom(i) => write!(f, "<{}>", i),
            Id(ty, a, b) => write!(f, "({} =[{}] {})", a, ty, b),
            Refl => f.write_str("refl"),
        }
    }
}

impl Display for Closure {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use Closure::*;
        let Plain(body) = self;
        body.fmt(f)
    }
}

impl Display for ConHead {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.name.text.fmt(f)
    }
}

impl Display for TermInfo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} at {}", self.ast, self.loc)
    }
}

fn pretty_application(
    f: &mut Formatter,
    fun: &impl Display,
    a: &[impl Display],
) -> Result<(), Error> {
    if a.is_empty() {
        fun.fmt(f)
    } else {
        write!(f, "({}", fun)?;
        for x in a {
            write!(f, " {}", x)?;
        }
        f.write_str(")")
    }
}
