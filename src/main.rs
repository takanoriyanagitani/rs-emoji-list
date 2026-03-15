use std::io;
use std::process::ExitCode;

use clap::Parser;

use rs_emoji_list::print_all_as_json2stdout;
use rs_emoji_list::print_all_simple_stdout;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
}

fn sub() -> Result<(), io::Error> {
    let cli = Cli::parse();
    match cli.verbose {
        false => print_all_simple_stdout(),
        true => print_all_as_json2stdout(),
    }
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
