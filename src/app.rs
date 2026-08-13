use crate::Args;
use crate::lox::Lox;
use std::process::ExitCode;

pub fn run_app(args: &Args) -> Result<ExitCode, String> {
    let mut lox = Lox::new();
    if let Some(filename) = args.filename.as_ref() {
        println!("{}", filename);
        lox.run_file(&filename)
    } else {
        lox.run_prompt()
    }
}
