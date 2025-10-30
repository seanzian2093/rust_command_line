use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::{Arg, ArgAction, Command};


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    let number_lines = config.number_lines;
    // dbg!(number_lines);
    let number_nonblank_lines = config.number_nonblank_lines;
    // dbg!(number_nonblank_lines);

    for filename in &config.files {
        // println!("{}", filename);
        // let f = open(filename)?;
        let f = open(filename);
        let mut number = 0;
        match f {
            Ok(file) => {
                for line in file.lines() {
                    let line_string = line?;

                    // when a line is empty
                    if line_string == "".to_string() {
                        // if -b is specified
                        // do not increase the line number and print empty line only
                        if number_nonblank_lines {
                            // number += 1;
                            println!("{}", line_string);
                            // if -n is specified
                            // increase the line number and print line number and empty line
                        } else if number_lines {
                            number += 1;
                            println!("{:6}\t{}", number, line_string);
                        } else {
                            println!("{}", line_string);
                        }
                        // if line is not empty, increase the line number and print accordingly
                    } else {
                        number += 1;
                        // if either -b or -n is specified, print line numbers
                        if number_lines || number_nonblank_lines {
                            println!("{:6}\t{}", number, line_string);
                        } else {
                            println!("{}", line_string);
                        }
                    }
                }
            },

            Err(err) => {
                eprintln!("Fail to open {}: {}", filename, err);
                continue;
            }
        }
    }
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch3-catr")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input files")
                .num_args(0..)
                .default_values(vec!["-"])
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number-lines")
                .help("Print number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank-lines")
                .help("Print number lines for nonblank lines")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    let files = matches.get_many::<String>("files")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();

    let number_lines = matches.get_flag("number_lines");
    let number_nonblank_lines = matches.get_flag("number_nonblank_lines");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}
