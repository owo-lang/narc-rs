use pest::Parser;
use pest_derive::Parser;

use crate::syntax::surf::{Expr, ExprDecl};
use voile_util::loc::Ident;
use voile_util::pest_util::end_of_rule;

#[derive(Parser)]
#[grammar = "syntax/surf/grammar.pest"]
struct NarcParser;

tik_tok!();

define_parse_str!(parse_str, NarcParser, file, decls, Vec<ExprDecl>);

fn decls(the_rule: Tok) -> Vec<ExprDecl> {
    the_rule.into_inner().into_iter().map(decl).collect()
}

fn decl(rules: Tok) -> ExprDecl {
    let mut inner: Tik = rules.into_inner();
    let the_rule: Tok = inner.next().unwrap();
    match the_rule.as_rule() {
        Rule::definition => definition(the_rule),
        Rule::clause => unimplemented!(),
        _ => unreachable!(),
    }
}

fn definition(rules: Tok) -> ExprDecl {
    let mut inner: Tik = rules.into_inner();
    let ident = next_ident(&mut inner);
    let expr = next_rule!(inner, expr);
    end_of_rule(&mut inner);
    ExprDecl::Defn(ident, expr)
}

fn expr(rules: Tok) -> Expr {
    unimplemented!()
}

#[inline]
fn next_ident(inner: &mut Tik) -> Ident {
    next_rule!(inner, ident)
}

fn ident(rule: Tok) -> Ident {
    Ident {
        text: rule.as_str().to_owned(),
        loc: From::from(rule.as_span()),
    }
}
