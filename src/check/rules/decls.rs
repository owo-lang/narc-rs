use voile_util::uid::GI;

use crate::check::monad::{TCM, TCS};
use crate::check::rules::data::check_data;
use crate::syntax::abs::AbsDecl;

const ERROR_TAKE: &str = "Please report this as a bug.";

pub fn check_decls(mut tcs: TCS, decls: Vec<AbsDecl>) -> TCM {
    let mut decls = decls.into_iter().map(Some).collect::<Vec<_>>();
    let range = 0..decls.len();
    let take = |decls: &mut [Option<AbsDecl>], i: usize| decls[i].take().expect(ERROR_TAKE);

    for i in range {
        if decls[i].is_none() {
            continue;
        }
        let decl = take(&mut decls, i);
        match decl {
            AbsDecl::Data(i) => {
                let cs = (i.conses.iter())
                    .map(|GI(j)| match take(&mut decls, *j) {
                        AbsDecl::Cons(i) => i,
                        _ => unreachable!(ERROR_TAKE),
                    })
                    .collect();
                tcs = check_data(tcs, i, cs)?;
            }
            AbsDecl::Cons(_) => unreachable!(ERROR_TAKE),
            _ => unimplemented!(),
        }
    }
    Ok(tcs)
}
