pub use self::ast::*;
pub use self::parse::*;

/// Surface syntax tree.
mod ast;
/// Parser, based on pest.
mod parse;

/// Parse a string into an optional declaration list and print error to stderr.
#[inline]
pub fn parse_str_err_printed(code: &str) -> Result<Vec<ExprDecl>, ()> {
    parse_str(code).map_err(|err| eprintln!("{}", err))
}

#[cfg(test)]
mod tests;
