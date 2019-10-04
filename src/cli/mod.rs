mod args;

const SUCCESS_MSG: &'static str = "\u{1F42E}\u{1F37A}";
const FAILURE_MSG: &'static str = "\u{1F528}";

fn main() {
    let args = args::pre();
    drop(args);
}
