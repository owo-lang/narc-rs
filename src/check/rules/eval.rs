use voile_util::meta::MetaSolution;

use crate::check::monad::{TermTCM, TCM, TCS};
use crate::syntax::abs::Abs;
use crate::syntax::common::Ductive;
use crate::syntax::core::{ConHead, Decl, Elim, Term};
use std::hint::unreachable_unchecked;
use voile_util::loc::Loc;

pub fn abs_to_elim(tcs: TCS, abs: Abs) -> TCM<(Elim, TCS)> {
    use Abs::*;
    match abs {
        Proj(ident, ix) => {
            debug_assert!(tcs.sigma.len() > ix.0);
            Ok((Elim::Proj(ident.text), tcs))
        }
        e => eval(tcs, e).map(|(t, tcs)| (Elim::app(t.ast), tcs)),
    }
}

pub fn eval(tcs: TCS, abs: Abs) -> TermTCM {
    use Abs::*;
    let view = abs.into_app_view();
    if !view.args.is_empty() {
        let (head, tcs) = eval(tcs, view.fun)?;
        let (elims, loc, tcs) = view.args.into_iter().try_fold(
            (vec![], head.loc, tcs),
            |(mut elims, o_loc, tcs), (loc, abs)| {
                let (elim, tcs) = abs_to_elim(tcs, abs)?;
                elims.push(elim);
                Ok((elims, o_loc + loc, tcs))
            },
        )?;
        return Ok((head.ast.apply_elim(elims).at(loc), tcs));
    }
    match view.fun {
        Type(ident, level) => Ok((Term::universe(level).at(ident.loc), tcs)),
        // Clearly eliminated by `into_app_view`.
        App(loc, f, a) => unsafe { unreachable_unchecked() },
        // Unlikely to desugar a thing like this, but I'm not sure.
        Proj(..) => unreachable!(),
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
        Meta(ident, mi) => {
            let sol = match tcs.meta_context.solution(mi) {
                MetaSolution::Solved(sol) => *sol.clone(),
                MetaSolution::Unsolved => Term::meta(mi, vec![]),
                MetaSolution::Inlined => unreachable!(),
            };
            Ok((sol.at(ident.loc), tcs))
        }
        Def(ident, def) => {
            debug_assert!(tcs.sigma.len() > def.0);
            Ok((Term::simple_def(def).at(ident.loc), tcs))
        }
        _ => unimplemented!(),
    }
}
