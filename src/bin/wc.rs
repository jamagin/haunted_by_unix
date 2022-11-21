use std::io::BufReader;
use std::fs::File;
use std::string::String;
use std::io;
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

struct Counts {
    chars: u64,
    lines: u64,
    words: u64,
}

fn count(handle) -> (Counts, io::Result<()>) {
    let (mut chars, mut lines, mut words) = (0, 0, 0);
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(handle);
    let mut buffer = [0; 10000000]; // XXX

    Counts {
        chars,
        lines,
        words,
    }
}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;
    if args.files.is_empty() {
        count(io::stdin());
    }
    for fname in args.files.iter() {
        let fhandle = File::open(fname).expect("failed to open");
        count(fhandle);
        // print lines words chars for this
        // accumulate
    }
    // print lines words chars total accumulated
    Ok(())
}
