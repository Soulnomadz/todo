use clap::Parser;
use todo::*;

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprint!("error: {}", e);
        std::process::exit(1);
    }
}
