use pest::Parser;
use pest_derive::Parser;

use voile_util::pest_util::end_of_rule;

#[derive(Parser)]
#[grammar = "syntax/surf/grammar.pest"]
struct NarcParser;

tik_tok!();
