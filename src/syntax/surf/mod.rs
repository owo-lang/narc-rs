pub use self::ast::*;
pub use self::parse::*;

/// Surface syntax tree.
mod ast;
/// Parser, based on pest.
mod parse;

/// Parse a string into an optional declaration list and print error to stderr.
#[inline]
pub fn parse_str_err_printed(code: &str) -> Option<Vec<ExprDecl>> {
    parse_str(code).map_err(|err| eprintln!("{}", err)).ok()
}

/// Parse a string into an optional standalone expression
/// and print error to stderr.
#[inline]
pub fn parse_expr_err_printed(code: &str) -> Option<Expr> {
    parse_str_expr(code)
        .map_err(|err| eprintln!("{}", err))
        .ok()
}

#[cfg(test)]
mod tests;
