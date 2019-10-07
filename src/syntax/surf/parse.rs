use pest::Parser;
use pest_derive::Parser;

use voile_util::loc::Ident;
use voile_util::pest_util::end_of_rule;

#[derive(Parser)]
#[grammar = "syntax/surf/grammar.pest"]
struct NarcParser;

tik_tok!();

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
