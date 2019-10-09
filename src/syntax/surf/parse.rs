use pest_derive::Parser;

use crate::syntax::core::ConHead;
use crate::syntax::pat::{Copat, Pat};
use crate::syntax::surf::{Expr, ExprCopat, ExprDecl, ExprPat, Param};
use voile_util::loc::Ident;
use voile_util::pest_util::{end_of_rule, strict_parse};
use voile_util::tags::Plicit;

#[derive(Parser)]
#[grammar = "syntax/surf/grammar.pest"]
struct NarcParser;

tik_tok!();

macro_rules! expr_parser {
    ($name:ident, $smaller:ident, $cons:ident) => {
        fn $name(rules: Tok) -> Expr {
            let mut exprs: Vec<Expr> = Default::default();
            for smaller in rules.into_inner() {
                exprs.push($smaller(smaller));
            }
            let first = exprs.remove(0);
            if exprs.is_empty() {
                first
            } else {
                Expr::$cons(first, exprs)
            }
        }
    };
}

pub fn parse_str(input: &str) -> Result<Vec<ExprDecl>, String> {
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
    end_of_rule(&mut inner);
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
        Rule::ident => Pat::Var(ident(the_rule)),
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
    let mut inner: Tik = rules.into_inner();
    let expr = next_rule!(inner, pi_expr);
    end_of_rule(&mut inner);
    expr
}

expr_parser!(dollar_expr, app_expr, app);

fn app_expr(rules: Tok) -> Expr {
    let mut inner: Tik = rules.into_inner();
    let fun = next_rule!(inner, primary_expr);
    let mut args = Vec::with_capacity(2);
    for expr in inner {
        args.push(applied(expr));
    }
    if args.is_empty() {
        fun
    } else {
        Expr::app(fun, args)
    }
}

fn applied(rules: Tok) -> Expr {
    let mut inner: Tik = rules.into_inner();
    let the_rule: Tok = inner.next().unwrap();
    match the_rule.as_rule() {
        Rule::dot_projection => Expr::Proj(dot_projection(the_rule)),
        Rule::primary_expr => primary_expr(the_rule),
        _ => unreachable!(),
    }
}

fn primary_expr(rules: Tok) -> Expr {
    let mut inner: Tik = rules.into_inner();
    let the_rule: Tok = inner.next().unwrap();
    let expr = match the_rule.as_rule() {
        Rule::ident => Expr::Var(ident(the_rule)),
        Rule::meta => Expr::Meta(meta(the_rule)),
        Rule::universe => Expr::Type(ident(the_rule)),
        Rule::expr => expr(the_rule),
        e => panic!("Unexpected rule: {:?} with token {}", e, the_rule.as_str()),
    };
    end_of_rule(&mut inner);
    expr
}

many_prefix_parser!(pi_expr_internal, Param, param, dollar_expr, Expr);
many_prefix_parser!(multi_param, Ident, ident, expr, Expr);

fn one_param(rules: Tok, licit: Plicit) -> Param {
    let mut inner: Tik = rules.into_inner();
    let (names, expr) = next_rule!(inner, multi_param);
    let ty = expr.unwrap();
    end_of_rule(&mut inner);
    Param { licit, names, ty }
}

fn pi_expr(rules: Tok) -> Expr {
    let (params, ret) = pi_expr_internal(rules);
    let ret = ret.unwrap();
    if params.is_empty() {
        ret
    } else {
        Expr::pi(params, ret)
    }
}

fn param(rules: Tok) -> Param {
    let mut inner: Tik = rules.into_inner();
    let the_rule: Tok = inner.next().unwrap();
    let param = match the_rule.as_rule() {
        Rule::explicit => one_param(the_rule, Plicit::Ex),
        Rule::implicit => one_param(the_rule, Plicit::Im),
        rule_type => Param {
            licit: Plicit::Ex,
            names: Vec::with_capacity(0),
            ty: match rule_type {
                Rule::dollar_expr => dollar_expr(the_rule),
                Rule::pi_expr => pi_expr(the_rule),
                e => panic!("Unexpected rule: {:?} with token {}", e, the_rule.as_str()),
            },
        },
    };
    end_of_rule(&mut inner);
    param
}

fn dot_projection(rules: Tok) -> Ident {
    parse_ident(rules)
}

fn meta(rules: Tok) -> Ident {
    parse_ident(rules)
}

fn parse_ident(rules: Tok) -> Ident {
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
