use super::{parse_expr_err_printed, parse_str_err_printed};

macro_rules! success {
    ($str:literal) => {
        parse_str_err_printed($str)
            .map(|ast| println!("{:?}", ast))
            .unwrap();
    };
}

macro_rules! success_expr {
    ($str:literal) => {
        parse_expr_err_printed($str)
            .map(|ast| println!("{:?}", ast))
            .unwrap();
    };
}

#[test]
fn definition_parse() {
    success!("");
    success!("definition test : Type;");
    success!("definition test : a b;");
    success!("definition test : a $ b c;");
}

#[test]
fn data_parse() {
    success!("data empty {};");
    success!("data unit { constructor tt; };");
    success!("data test tele {};");
    success!("data test tele { constructor tt tele; };");
    success!("data test (x : tele) { constructor tt (y : x); };");
}

#[test]
fn codata_parse() {
    success!("codata unit {};");
    success!("codata unit2 { projection tt : unit; };");
    success!("codata test tele {};");
    success!("codata test tele { projection tt : tele; };");
    success!("codata test (x : tele) { projection tt : (y : x) -> y; };");
}

#[test]
fn expr_parse() {
    success_expr!("Type");
    success_expr!("a b");
    success_expr!("a $ b c");
}

#[test]
fn clause_parse() {
    success!("");
    success!("clause test = Type;");
    success!("clause test .a = a;");
    success!("clause test a = a;");
    success!("clause test (a b) = b;");
    success!("clause test |_a b_| = b;");
    success!("clause test |_a b_| c = b;");
    success!("clause test |_a b_| (c d) = b;");
    success!("clause test |_a b_| .c = b;");
    success!("clause test |_a b_| .c (c d) = b;");
    success!("clause test .e |_a b_| .c (c d) = b;");
}
