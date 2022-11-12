use std::{string::String, io::Stdin};
use lexopt::prelude::*;
// for now only implementing -c -l -m -w
struct Args {
    // bytes: bool,
    chars: bool,
    lines: bool,
    words: bool,
    files: Vec<String>,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    let mut chars = false;
    let mut lines = false;
    let mut words = false;
    let mut files: Vec<String> = Vec::new();
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('c') => {
                chars = true;
            }
            Short('l') => {
                lines = true;
            }
            Short('w') => {
                words = true;
            }
            Value(val) => {
                files.push(val.into_string()?);
            }
            Long("help") => {
                println!("Usage: wc -clw [file ...]");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    if !chars && !lines && !words {
        // no mode selected, emit all
        chars = true;
        lines = true;
        words = true;
    }

    Ok(Args {
        chars,
        lines,
        words,
        files,
    })
}

fn count(fname) -> (lines, words, chars) {

}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;
    if args.files.is_empty() {
        count(Stdin);
    }
    for fname in args.files.iter() {
        count(fname);
        // print lines words chars for this
        // accumulate
    }
    // print lines words chars total accumulated
    Ok(())
}
