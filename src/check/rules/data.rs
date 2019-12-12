use crate::check::monad::{TCM, TCS};
use crate::check::rules::term::check;
use crate::syntax::abs::{AbsConsInfo, AbsDataInfo, AbsTele};
use crate::syntax::core::{ConsInfo, DataInfo, Decl, Val};

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

fn check_cons(tcs: TCS, cons: AbsConsInfo, ty: &Val) -> TCM<(TCS, ConsInfo)> {
    let param_len = tcs.gamma.len();
    let mut tcs = check_tele(tcs, cons.tele, ty)?;
    let info = ConsInfo {
        loc: cons.source,
        name: cons.name,
        params: tcs.gamma.split_off(param_len),
        data: cons.data_ix,
        // Inductive!
        fields: None,
    };
    Ok((tcs, info))
}

pub fn check_data(tcs: TCS, data: AbsDataInfo, conses: Vec<AbsConsInfo>) -> TCM {
    let t = Val::Type(data.level);
    let mut tcs = check_tele(tcs, data.tele, &t)?;
    let param_len = tcs.gamma.len();

    let info = DataInfo {
        params: tcs.gamma.clone(),
        loc: data.source,
        name: data.name,
        level: data.level,
        conses: data.conses,
    };
    tcs.sigma.push(Decl::Data(info));

    // For debugging only.
    let mut data_ix = None;

    for cons in conses {
        let (new_tcs, cons) = check_cons(tcs, cons, &t)?;
        tcs = new_tcs;
        match data_ix {
            None => data_ix = Some(cons.data),
            Some(ix) => debug_assert_eq!(ix, cons.data),
        }
        debug_assert_eq!(param_len, tcs.gamma.len());

        tcs.sigma.push(Decl::Cons(cons));
    }
    tcs.gamma.clear();
    Ok(tcs)
}
