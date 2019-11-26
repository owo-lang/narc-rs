use voile_util::meta::MI;
use voile_util::uid::{GI, UID};

use crate::syntax::abs::desugar::desugar_main;
use crate::syntax::abs::{Abs, AbsClause, AbsCopat, AbsDecl};
use crate::syntax::pat::{Copat, Pat};
use crate::syntax::surf::parse_str;

fn expect_clause(def: AbsDecl) -> AbsClause {
    if let AbsDecl::Clause(c) = def {
        c
    } else {
        panic!("Test failed")
    }
}

fn expect_var_expr(p: Abs) -> UID {
    if let Abs::Var(_, uid) = p {
        uid
    } else {
        panic!("Test failed")
    }
}

fn expect_app_var_pat(p: AbsCopat) -> UID {
    if let Copat::App(Pat::Var(uid)) = p {
        uid
    } else {
        panic!("Test failed")
    }
}

#[test]
fn simple_pattern_definition_desugar() {
    let code = "\
    definition test : (a : Type) -> Type a;
    clause test a = a;
    ";
    let mut state = desugar_main(parse_str(code).unwrap()).unwrap();
    println!("{:#?}", state);
    assert!(state.local.is_empty());
    let mut c = expect_clause(state.decls.remove(1));
    assert_eq!(c.patterns.len(), 1);
    assert_eq!(
        expect_app_var_pat(c.patterns.remove(0)),
        expect_var_expr(c.body)
    );
}

#[test]
fn simple_definition_desugar() {
    let code = "\
    definition test : Type;
    clause test = Type;
    ";
    let mut state = desugar_main(parse_str(code).unwrap()).unwrap();
    println!("{:#?}", state);
    assert!(state.local.is_empty());
    assert_eq!(state.decls.len(), 2);
    assert_eq!(state.meta_count, MI(0));
    let c = expect_clause(state.decls.remove(1));
    assert_eq!(c.definition, GI(0));
    assert!(c.patterns.is_empty());
}
