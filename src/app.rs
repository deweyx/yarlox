use crate::Args;

use std::process::ExitCode;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn run(args: &Args) -> Result<ExitCode, String> {
    let mut file_reader = BufReader::new(File::open(&args.filename).map_err(|_| "Unable to open file")?);

    let mut buf: String = String::default();
    let len = file_reader.read_to_string(&mut buf) .map_err(|_| "Unable to read from file")?;
    if len == 0 {
        println!("-- empty file --");
    } else {
        println!("{buf}");
    };
    Ok(ExitCode::from(42))
}