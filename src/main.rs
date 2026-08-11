mod app;

use clap::Parser;
use std::process::ExitCode;

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Args {
    #[clap(default_value = "-")]
    filename: String,
}

fn main() -> Result<ExitCode, String> {
    let args = Args::parse();
    app::run(&args)
}
