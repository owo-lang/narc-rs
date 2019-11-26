use voile_util::meta::MI;
use voile_util::uid::GI;

use crate::syntax::abs::desugar::desugar_main;
use crate::syntax::abs::AbsDecl;
use crate::syntax::surf::parse_str;

#[test]
fn simple_definition_desugar() {
    let code = "\
    definition test : Type;
    clause test = Type;
    ";
    let parsed = parse_str(code).unwrap();
    let state = desugar_main(parsed).unwrap();
    println!("{:#?}", state);
    assert!(state.local.is_empty());
    assert_eq!(state.decls.len(), 2);
    assert_eq!(state.meta_count, MI(0));
    if let AbsDecl::Clause(c) = &state.decls[1] {
        assert_eq!(c.definition, GI(0));
    } else {
        panic!("Test failed")
    }
}
