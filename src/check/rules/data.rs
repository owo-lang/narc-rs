use crate::check::monad::{TCM, TCS};
use crate::check::rules::term::check;
use crate::syntax::abs::{AbsConsInfo, AbsDataInfo, AbsTele};
use crate::syntax::core::{ConsInfo, DataInfo, Val};

/// The checked tele is put into the returned `tcs.gamma`.
pub fn check_tele(mut tcs: TCS, tele: AbsTele, ty: &Val) -> TCM {
    for bind in tele {
        let (checked, new_tcs) = check(tcs, &bind.ty, ty)?;
        tcs = new_tcs;
        let bind = bind.map_term(|_| checked.ast);
        tcs.gamma.push(bind);
    }
    Ok(tcs)
}

pub fn check_cons(tcs: TCS, cons: AbsConsInfo, ty: &Val) -> TCM<(TCS, ConsInfo)> {
    let param_len = tcs.gamma.len();
    let mut tcs = check_tele(tcs, cons.tele, ty)?;
    let info = ConsInfo {
        loc: cons.source,
        name: cons.name.text,
        params: tcs.gamma.split_off(param_len),
        data: cons.data_ix,
        // Inductive!
        fields: None,
    };
    Ok((tcs, info))
}

pub type DataTCS = (TCS, DataInfo, Vec<ConsInfo>);

pub fn check_data(tcs: TCS, data: AbsDataInfo, conses: Vec<AbsConsInfo>) -> TCM<DataTCS> {
    let t = Val::Type(data.level);
    let mut tcs = check_tele(tcs, data.tele, &t)?;
    let param_len = tcs.gamma.len();
    // For debugging only.
    let mut data_ix = None;
    let mut cons_collect = Vec::with_capacity(conses.len());
    for cons in conses {
        let (new_tcs, cons) = check_cons(tcs, cons, &t)?;
        tcs = new_tcs;
        match data_ix {
            None => data_ix = Some(cons.data),
            Some(ix) => debug_assert_eq!(ix, cons.data),
        }
        debug_assert_eq!(param_len, tcs.gamma.len());
        cons_collect.push(cons);
    }
    let mut params = vec![];
    std::mem::swap(&mut params, &mut tcs.gamma);
    let info = DataInfo {
        params,
        loc: data.source,
        name: data.name.text,
        level: data.level,
        conses: data.conses,
    };
    Ok((tcs, info, cons_collect))
}
