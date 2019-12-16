use nar::check::monad::TCS;
use nar::check::rules::check_decls;
use nar::syntax::abs::desugar::{desugar_main, DesugarState};

use crate::args::CliOptions;

mod args;
mod util;

fn success(quiet: bool) {
    if !quiet {
        println!("\u{1F42E}\u{1F37A}");
    }
}

fn main_file(args: CliOptions) -> Option<(TCS, DesugarState)> {
    let quiet = args.quiet;
    let parse_only = args.parse_only;
    let indentation = args.indent_size.unwrap_or(2);
    let decls = util::parse_file(args.file.as_ref()?)?;
    if parse_only {
        success(quiet);
        return None;
    }

    // Translate to abstract syntax
    let abs_decls = desugar_main(decls).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprint!("\u{1f375}");
        std::process::exit(1)
    });

    // Type check
    let mut tcs = TCS::considerate_of(&abs_decls);
    tcs.indentation_size(indentation);
    tcs.trace_tc = args.trace;
    let checked = check_decls(tcs, abs_decls.decls.clone()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("\u{1F528}");
        std::process::exit(1)
    });

    success(quiet);
    Some((checked, abs_decls))
}

fn main() {
    let args = args::pre();

    let checked = main_file(args).unwrap_or_default();

    // Don't yet need to use this -- it's for the REPL.
    drop(checked);
}
