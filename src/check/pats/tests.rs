use crate::check::monad::TCS;
use crate::check::rules::{check_decls, simplify};
use crate::syntax::abs::desugar::desugar_main;
use crate::syntax::core::subst::DeBruijn;
use crate::syntax::core::Decl;
use crate::syntax::surf::parse_str;
use voile_util::uid::DBI;

#[test]
fn simple_simplify() {
    let code = "\
    definition id : {A : Type} -> A -> A;
    clause id a = a;

    definition id' : {A : Type} -> A -> A;
    clause id' a = id a;
    ";
    let desugar = desugar_main(parse_str(code).unwrap()).unwrap();
    let tcs = check_decls(TCS::considerate_of(&desugar), desugar.decls).unwrap();
    let id_def = match &tcs.sigma[2] {
        Decl::Func(f) => f.clone(),
        _ => panic!(),
    };
    let body = id_def.clauses[0].body.clone().unwrap();
    let (body, _tcs) = simplify(tcs, body).unwrap();
    assert_eq!(body, DeBruijn::from_dbi(DBI(1)))
}
