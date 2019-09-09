mod args;

fn main() {
    let args = args::pre();
    drop(args);
}
