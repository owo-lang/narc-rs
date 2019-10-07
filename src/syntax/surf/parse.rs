use pest_derive::Parser;

use crate::syntax::core::ConHead;
use crate::syntax::pat::{Copat, Pat};
use crate::syntax::surf::{Expr, ExprCopat, ExprDecl, ExprPat};
use voile_util::loc::Ident;
use voile_util::pest_util::{end_of_rule, strict_parse};

#[derive(Parser)]
#[grammar = "syntax/surf/grammar.pest"]
struct NarcParser;

tik_tok!();

pub fn parse_str_expr(input: &str) -> Result<Vec<ExprDecl>, String> {
    strict_parse::<NarcParser, _, _, _>(Rule::file, input, decls)
}

fn decls(the_rule: Tok) -> Vec<ExprDecl> {
    the_rule.into_inner().into_iter().map(decl).collect()
}

fn decl(rules: Tok) -> ExprDecl {
    let mut inner: Tik = rules.into_inner();
    let the_rule: Tok = inner.next().unwrap();
    match the_rule.as_rule() {
        Rule::definition => definition(the_rule),
        Rule::clause => clause(the_rule),
        _ => unreachable!(),
    }
}

many_prefix_parser!(clause_body, ExprCopat, copattern, expr, Expr);

fn clause(rules: Tok) -> ExprDecl {
    let mut inner: Tik = rules.into_inner();
    let ident = next_ident(&mut inner);
    let (copats, expr) = next_rule!(inner, clause_body);
    ExprDecl::Cls(ident, copats, expr.unwrap())
}

fn definition(rules: Tok) -> ExprDecl {
    let mut inner: Tik = rules.into_inner();
    let ident = next_ident(&mut inner);
    let expr = next_rule!(inner, expr);
    end_of_rule(&mut inner);
    ExprDecl::Defn(ident, expr)
}

fn copattern(rules: Tok) -> ExprCopat {
    let mut inner: Tik = rules.into_inner();
    let the_rule: Tok = inner.next().unwrap();
    match the_rule.as_rule() {
        Rule::pattern => Copat::App(pattern(the_rule)),
        Rule::dot_projection => Copat::Proj(dot_projection(the_rule).text),
        _ => unreachable!(),
    }
}

fn pattern(rules: Tok) -> ExprPat {
    let mut inner: Tik = rules.into_inner();
    let the_rule: Tok = inner.next().unwrap();
    match the_rule.as_rule() {
        Rule::inacc_pat => inacc_pat(the_rule),
        Rule::cons_pat => cons_pat(the_rule),
        Rule::ident => Pat::Var(next_ident(&mut the_rule.into_inner())),
        _ => unreachable!(),
    }
}

fn cons_pat(rules: Tok) -> ExprPat {
    let mut inner: Tik = rules.into_inner();
    let ident = next_ident(&mut inner);
    let pats = inner.into_iter().map(pattern).collect();
    Pat::Cons(false, ConHead::pseudo(ident.text), pats)
}

fn inacc_pat(rules: Tok) -> ExprPat {
    let mut inner: Tik = rules.into_inner();
    let expr = next_rule!(inner, expr);
    end_of_rule(&mut inner);
    Pat::Forced(expr)
}

fn expr(rules: Tok) -> Expr {
    unimplemented!()
}

fn dot_projection(rules: Tok) -> Ident {
    let mut inner: Tik = rules.into_inner();
    let ident = next_ident(&mut inner);
    end_of_rule(&mut inner);
    ident
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
