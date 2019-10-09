use super::parse_str_err_printed;

macro_rules! success {
    ($str:literal) => {
        parse_str_err_printed($str)
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
