use std::str;

use minitt_util::io::read_file;

use nar::syntax::surf::{parse_str_err_printed, ExprDecl};

pub fn parse_file(file_arg: &str) -> Option<Vec<ExprDecl>> {
    // If cannot read input, return.
    let file_content = read_file(file_arg)?;
    // Read file
    let file_content_utf8 = str::from_utf8(file_content.as_slice()).unwrap();
    // Parse
    parse_str_err_printed(file_content_utf8)
}
