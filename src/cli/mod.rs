use nar::check::monad::TCS;
use nar::check::rules::check_decls;
use nar::syntax::abs::desugar::{desugar_main, DesugarState};

mod args;
mod util;

const SUCCESS_MSG: &'static str = "\u{1F42E}\u{1F37A}";
const FAILURE_MSG: &'static str = "\u{1F528}";

fn main_file(
    file_ref: Option<&String>,
    quiet: bool,
    parse_only: bool,
) -> Option<(TCS, DesugarState)> {
    let decls = util::parse_file(file_ref?)?;
    if !quiet {
        println!("Parse successful.");
    }
    if parse_only {
        return None;
    }

    // Translate to abstract syntax
    let abs_decls = desugar_main(decls).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("{}", FAILURE_MSG);
        std::process::exit(1)
    });

    // Type Check
    let mut tcs = TCS::default();
    tcs.meta_context
        .expand_with_fresh_meta(abs_decls.meta_count);
    let checked = check_decls(tcs, abs_decls.decls.clone()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("{}", FAILURE_MSG);
        std::process::exit(1)
    });

    if !quiet {
        println!("{}", SUCCESS_MSG);
    }

    Some((checked, abs_decls))
}

fn main() {
    let args = args::pre();

    let checked = main_file(args.file.as_ref(), args.quiet, args.parse_only).unwrap_or_default();

    // Don't yet need to use this -- it's for the REPL.
    drop(checked);
}
