use super::parse_str_err_printed;

macro_rules! success {
    ($str:literal) => {
        parse_str_err_printed($str)
            .map(|ast| println!("{:?}", ast))
            .unwrap();
    };
}

#[test]
fn simple_parse() {
    success!("");
    success!("definition test : Type;");
    success!("definition test : a b;");
    success!("definition test : a $ b c;");
}
