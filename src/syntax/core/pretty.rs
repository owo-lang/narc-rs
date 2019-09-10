use std::fmt::{Display, Error, Formatter};

use voile_util::tags::Plicit;

use super::{CaseSplit, Closure, Neutral, Val, ValInfo};

impl Display for Neutral {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use Neutral::*;
        match self {
            Var(dbi) => write!(f, "[{}]", dbi),
            // This might be conflict with other syntax.
            Ref(dbi) => write!(f, "[|{}|]", dbi),
            Axi(a) => a.fmt(f),
            Meta(mi) => write!(f, "?{}", mi),
            App(fun, a) => {
                write!(f, "({}", fun)?;
                for x in a {
                    write!(f, " {}", x)?;
                }
                f.write_str(")")
            }
            SplitOn(split, on) => {
                write!(f, "(case {} of {{ ", on)?;
                pretty_split(f, &split)?;
                f.write_str("})")
            }
            Lift(levels, p) => write!(f, "(^[{:?}] {})", levels, p),
        }
    }
}

impl Display for Closure {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use Closure::*;
        match self {
            Plain(body) => body.fmt(f),
            Tree(split) => {
                for (label, closure) in split {
                    write!(f, "{} => {}; ", label, closure)?;
                }
                Ok(())
            }
        }
    }
}

impl Display for ValInfo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} at {}", self.ast, self.loc)
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Val::Type(l) => write!(f, "set{}", l),
            Val::Lam(clos) => write!(f, "(\\ {})", clos),
            Val::Pi(Plicit::Ex, param_ty, clos) => write!(f, "({} -> {})", param_ty, clos),
            Val::Pi(Plicit::Im, param_ty, clos) => write!(f, "({{{}}} -> {})", param_ty, clos),
            Val::Neut(neut) => neut.fmt(f),
            Val::Cons(name, a) => write!(f, "(@{} {})", name, a),
        }
    }
}

fn pretty_split(f: &mut Formatter, split: &CaseSplit) -> Result<(), Error> {
    for (name, closure) in split {
        write!(f, "{}: \\ {}; ", name, closure)?;
    }
    Ok(())
}
