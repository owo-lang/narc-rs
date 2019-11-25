use crate::check::monad::{TCM, TCS};
use crate::check::rules::check;
use crate::syntax::abs::{AbsDataInfo, AbsTele};
use crate::syntax::core::{DataInfo, Term, TYPE_OMEGA};

/// The checked tele is put into the returned `tcs.gamma`.
pub fn check_tele(mut tcs: TCS, tele: AbsTele) -> TCM<TCS> {
    for bind in tele {
        let (checked, new_tcs) = check(tcs, &bind.ty, &TYPE_OMEGA)?;
        tcs = new_tcs;
        let bind = bind.map_term(|_| checked.ast);
        tcs.gamma.push(bind);
    }
    Ok(tcs)
}

pub fn check_data(tcs: TCS, data: AbsDataInfo) -> TCM<(TCS, DataInfo)> {
    let t = Term::universe(data.level);
    let tcs = check_tele(tcs, data.tele)?;
    let param_len = tcs.gamma.len();
    // TODO: check conses
    unimplemented!()
}
