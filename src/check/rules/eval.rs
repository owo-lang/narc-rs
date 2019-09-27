use crate::check::monad::{TermTCM, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::common::Ductive;
use crate::syntax::core::{ConHead, Decl, Term};

pub fn eval(tcs: TCS, abs: Abs) -> TermTCM {
    use Abs::*;
    match abs {
        Type(ident, level) => Ok((Term::universe(level).at(ident.loc), tcs)),
        App(loc, f, a) => {
            let (f, tcs) = eval(tcs, *f)?;
            let (a, tcs) = eval(tcs, *a)?;
            Ok((f.ast.apply(vec![a.ast]).at(loc), tcs))
        }
        Cons(ident, ix) => {
            let fields = match tcs.def(ix) {
                Decl::Cons { fields, .. } => fields,
                _ => unreachable!(),
            };
            let head = ConHead {
                name: ident.text,
                ductive: match fields {
                    None => Ductive::In,
                    Some(_) => Ductive::Coin,
                },
                fields: fields.clone().unwrap_or_default(),
            };
            Ok((Term::cons(head, vec![]).at(ident.loc), tcs))
        }
        _ => unimplemented!(),
    }
}
