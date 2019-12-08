use std::fmt::{Display, Error, Formatter};

use voile_util::tags::Plicit;
use Plicit::{Ex as Explicit, Im as Implicit};

use crate::syntax::abs::{Abs, Bind};

impl Display for Abs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use Abs::*;
        match self {
            Def(id, _gi) => write!(f, "{}", id.text),
            Var(id, _uid) => write!(f, "{}", id.text),
            Meta(id, _mi) => write!(f, "?{}", id.text),
            App(a, args) => {
                write!(f, "({} {}", a, args.head())?;
                for arg in args.tail() {
                    write!(f, " {}", arg)?;
                }
                f.write_str(")")
            }
            Pi(_loc, Bind { licit, ty, .. }, clos) => match licit {
                Explicit => write!(f, "({} -> {})", ty, clos),
                Implicit => write!(f, "({{{}}} -> {})", ty, clos),
            },
            Type(_, l) => write!(f, "Type{}", l),
            Cons(id, _gi) => write!(f, "{}", id.text),
            Proj(id, _gi) => write!(f, "{}", id.text),
        }
    }
}
