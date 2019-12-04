use voile_util::meta::MI;
use voile_util::uid::{GI, UID};

use crate::syntax::abs::desugar::desugar_main;
use crate::syntax::abs::*;
use crate::syntax::pat::{Copat, Pat};
use crate::syntax::surf::parse_str;

macro_rules! make_expect_decl {
    ($name:ident, $pat:ident, $ret:ty) => {
        fn $name(def: AbsDecl) -> $ret {
            if let AbsDecl::$pat(c) = def {
                c
            } else {
                panic!("Test failed")
            }
        }
    };
}

make_expect_decl!(expect_clause, Clause, AbsClause);
make_expect_decl!(expect_data, Data, AbsDataInfo);
make_expect_decl!(expect_constructor, Cons, AbsConsInfo);

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
fn simple_data_and_constructor_desugar() {
    let code = "\
    data test {
      constructor test-cons;
    };
    ";
    let mut state = desugar_main(parse_str(code).unwrap()).unwrap();
    println!("{:#?}", state);
    assert_eq!(state.decls.len(), 2);
    assert!(state.local.is_empty());
    let cons: AbsConsInfo = expect_constructor(state.decls.remove(1));
    let data: AbsDataInfo = expect_data(state.decls.remove(0));
    assert!(data.tele.is_empty());
    assert!(cons.tele.is_empty());
    assert_eq!(cons.name.text, "test-cons");
    assert_eq!(data.name.text, "test");
    assert_eq!(data.conses, vec![GI(1)]);
    assert_eq!(cons.data_ix, GI(0));
}

#[test]
fn multiple_patterns_increases_ix() {
    let code = "\
    definition test : (a : Type) -> Type a;
    clause test a = a;
    clause test a = a;

    definition test2 : (a : Type) -> Type a;
    clause test2 a = a;
    ";
    let mut state = desugar_main(parse_str(code).unwrap()).unwrap();
    println!("{:#?}", state);
    assert!(state.local.is_empty());
    assert_eq!(state.decls.len(), 5);
    let mut c: AbsClause = expect_clause(state.decls.remove(4));
    assert_eq!(c.patterns.len(), 1);
    assert_eq!(c.definition, GI(3));
    assert_eq!(
        expect_app_var_pat(c.patterns.remove(0)),
        expect_var_expr(c.body)
    );
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
    assert_eq!(state.decls.len(), 2);
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
