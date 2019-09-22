use crate::check::monad::{TermTCM, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::core::{Term, TermInfo};

pub fn eval(tcs: TCS, abs: Abs) -> TermTCM {
    use Abs::*;
    match abs {
        Type(ident, level) => Ok((Term::universe(level).into_info(ident.loc), tcs)),
        App(loc, f, a) => {
            let (f, tcs) = eval(tcs, *f)?;
            let (a, tcs) = eval(tcs, *a)?;
            unimplemented!()
        }
        _ => unimplemented!(),
    }
}
