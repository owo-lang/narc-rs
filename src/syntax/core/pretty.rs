use std::fmt::{Display, Error, Formatter};

use voile_util::tags::Plicit;

use super::{Closure, Elim, Term, Val, ValInfo};

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
            Term::Redex(fun, args) => pretty_application(f, fun, args),
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
            App(fun, a) => pretty_application(f, fun, a),
            Type(l) => write!(f, "set{}", l),
            Pi(Plicit::Ex, param_ty, clos) => write!(f, "({} -> {})", param_ty, clos),
            Pi(Plicit::Im, param_ty, clos) => write!(f, "({{{}}} -> {})", param_ty, clos),
            Cons(name, a) => pretty_application(f, name, a),
            Data(kind, params) => unimplemented!(),
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

impl Display for ValInfo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} at {}", self.ast, self.loc)
    }
}

fn pretty_application(
    f: &mut Formatter,
    fun: &impl Display,
    a: &[impl Display],
) -> Result<(), Error> {
    write!(f, "({}", fun)?;
    for x in a {
        write!(f, " {}", x)?;
    }
    f.write_str(")")
}
