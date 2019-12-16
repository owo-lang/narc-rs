use voile_util::uid::{DBI, GI};

use crate::{
    check::{
        monad::TCS,
        rules::{check_decls, simplify},
    },
    syntax::{
        abs::desugar::desugar_main,
        core::{subst::DeBruijn, Decl},
        surf::parse_str,
    },
};

#[test]
fn simple_simplify() {
    let code = "\
    definition id : {A : Type} -> A -> A;
    clause id a = a;

    definition id' : {A : Type} -> A -> A;
    clause id' a = id a;
    ";
    let desugar = desugar_main(parse_str(code).unwrap()).unwrap();
    let mut tcs = check_decls(TCS::considerate_of(&desugar), desugar.decls).unwrap();
    let id_def = match tcs.take_sigma(GI(2)) {
        Decl::Func(f) => f,
        _ => panic!(),
    };
    let body = id_def.clauses[0].body.clone().unwrap();
    let (body, _tcs) = simplify(tcs, body).unwrap();
    assert_eq!(body, DeBruijn::from_dbi(DBI(1)))
}
