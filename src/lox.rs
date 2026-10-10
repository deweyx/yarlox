use crate::scanner;
use crate::scanner::{Scanner, ScannerResult};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::ExitCode;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn report_error(&mut self, line: usize, locat: String, message: String) {
        println!("[lox line {line}] Error{locat}: {message}");
        self.had_error = true;
    }

    pub fn run_file(&mut self, filename: &str) -> Result<ExitCode, String> {
        let mut file_reader =
            BufReader::new(File::open(&filename).map_err(|_| "Unable to open file")?);

        let mut buf: String = String::default();
        let len = file_reader
            .read_to_string(&mut buf)
            .map_err(|_| "Unable to read from file")?;
        if len == 0 {
            println!("-- empty file --");
        } else {
            println!("{buf}");
        };
        if self.had_error {
            std::process::exit(65);
        }
        Ok(ExitCode::SUCCESS)
    }

    pub fn run_prompt(&mut self) -> Result<ExitCode, String> {
        let stdin = std::io::stdin();
        loop {
            print!("yarlox> ");
            flush()?;

            let mut line = String::new();
            stdin.read_line(&mut line)
                .map_err(|_| "Unable to read from stdin")?;
            if line.is_empty() {
                break;
            }

            if !line.trim().is_empty() {
                if let Err(e) = Self::run(&line) {
                    println!("{e:?}");
                }
            }
        }
        Ok(ExitCode::SUCCESS)
    }

    pub fn run(source: &str) -> ScannerResult<()> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        // For now, just print the tokens.
        for token in tokens {
            println!("{token:?}");
        }
        Ok(())
    }
}

fn flush() -> Result<(), String> {
    std::io::stdout()
        .flush()
        .map_err(|_| "Unable to flush stdout")?;
    Ok(())
}
