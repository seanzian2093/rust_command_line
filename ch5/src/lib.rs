#![allow(unused)]
use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch5-wcr")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                // .multiple(true)
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("Show line count")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("Show word count")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Show byte count")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("Show byte count")
                .conflicts_with("bytes")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let files = matches.get_many::<String>("files")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();

    let lines = matches.get_flag("lines");
    let words = matches.get_flag("words");
    let bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");


    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    // println!("{:#?}", config);
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(file_info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(file_info.num_lines, config.lines),
                        format_field(file_info.num_words, config.words),
                        format_field(file_info.num_bytes, config.bytes),
                        format_field(file_info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );

                    total_lines += file_info.num_lines;
                    total_words += file_info.num_words;
                    total_bytes += file_info.num_bytes;
                    total_chars += file_info.num_chars;
                }
            }
        }
    }

    // In case of multiple input files
    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        );
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes= 0;
    let mut num_chars = 0;

    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        // if bytes is 0, all is 0, break the loop and return all 0s
        if line_bytes == 0 {
            break;
        }
        // otherwise
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        // reuse line in a loop, critical to clear() after an iteration
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}