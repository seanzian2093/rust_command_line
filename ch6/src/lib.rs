#![allow(unused)]
use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch6-uniqr")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust uniq")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                // .multiple(true)
                .num_args(1)
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Show count")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let in_file = matches.get_one::<String>("in_file").unwrap().clone();
    let out_file = matches.get_one::<String>("out_file").map(String::from);
    let count = matches.get_flag("count");


    Ok(Config { in_file, out_file, count})
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}


pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    // Define a function to write to output file
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    // Defining a function inside another one is not allowed. use closure instead
    let mut print = |count: u64, text: &str| {
        if count > 0 {
            if config.count {
                // print!("{:>4} {}", count, text);
                write!(out_file, "{:>4} {}", count, text);
            } else {
                write!(out_file, "{}", text);
            }
        };
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        // A new line detected
        if line.trim_end() != previous.trim_end() {
            // Print out previous
            print(count, &previous);
            // Update previous
            previous = line.clone();
            // Reset count
            count = 0
        }
        // Existing line, increase the count and clear buffer and onto next line
        count += 1;
        line.clear();
    }
    // last line
    print(count, &previous);

    Ok(())
}