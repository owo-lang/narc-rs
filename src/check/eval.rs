use crate::syntax::abs::Abs;
use crate::syntax::core::{Term, TermInfo};

use super::monad::TCS;

pub fn eval(tcs: TCS, abs: Abs) -> (TermInfo, TCS) {
    use Abs::*;
    match abs {
        Type(ident, level) => (Term::universe(level).into_info(ident.loc), tcs),
        _ => unimplemented!(),
    }
}
