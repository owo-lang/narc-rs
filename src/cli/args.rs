use clap::{App, AppSettings};
use minitt_util::cli::{cli_completion_generation, GenShellSubCommand};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    about,
    name = "narc",
    global_settings(&[AppSettings::ColoredHelp])
)]
pub struct CliOptions {
    /// the input file to type-check (Notice: file should be UTF-8 encoded)
    #[structopt(name = "FILE")]
    pub file: Option<String>,

    /// Parses but do not type-check the input file
    #[structopt(short = "p", long)]
    pub parse_only: bool,

    /// Prints errors only
    #[structopt(short = "q", long)]
    pub quiet: bool,

    #[structopt(subcommand)]
    completion: Option<GenShellSubCommand>,
}

fn app<'a, 'b>() -> App<'a, 'b> {
    let extra_help = "\
    Narc will not load the file if parse failed, \
    and will say \u{1f336}\u{1f414} if scope-check failed, \
    and \u{1F528} if type-check failed.
    If type-check succeeded, Narc will say \u{1F42E}\u{1F37A}.\n
    For extra help please head to \
    https://github.com/owo-lang/narc-rs/issues/new";
    // Introduced a variable because stupid CLion :(
    let app: App = CliOptions::clap();
    app.after_help(extra_help)
}

pub fn pre() -> CliOptions {
    let args: CliOptions = CliOptions::from_clap(&app().get_matches());
    cli_completion_generation(&args.completion, app);
    args
}
