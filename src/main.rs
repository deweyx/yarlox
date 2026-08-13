mod app;
mod lox;
mod scanner;
mod token;

use clap::Parser;
use std::process::ExitCode;

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct Args {
    filename: Option<String>,
}

fn main() -> Result<ExitCode, String> {
    let args = Args::parse();
    app::run_app(&args)
}
