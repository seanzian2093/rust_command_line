use clap::{Command, Arg, ArgAction};
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::mem;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}


pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch9-grepr")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust grep")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true)
                .num_args(1)
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .help("Selected field")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Count occurrences")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("invert")
                .short('v')
                .long("invert-match")
                .help("Invert-match")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let pattern= matches.get_one::<String>("pattern").unwrap();
    let pattern = RegexBuilder::new(pattern)
        .case_insensitive(matches.get_flag("insensitive"))
        .build()
        .map_err(|_| format!("Invalid pattern \"{}\"", pattern))?;

    let files = matches.get_many::<String>("files")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();

    let recursive = matches.get_flag("recursive");
    let count = matches.get_flag("count");
    let invert_match= matches.get_flag("invert");

    Ok(Config{
        pattern,
        files,
        recursive,
        count,
        invert_match,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let entries = find_files(&config.files, config.recursive);
    let num_files = entries.len();
    let print = |fname: &str, val: &str| {
        if num_files > 1 {
            print!("{fname}:{val}");
        } else {
            print!("{val}");
        }
    };

    for entry in entries {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(filename) => match open(&filename) {
                Err(e) => eprintln!("{}: {}", filename, e),
                Ok(file) => {
                    match find_lines(
                    // match _find_lines(
                        file,
                        &config.pattern,
                        config.invert_match,
                    ) {
                        Err(e) => eprintln!("{}", e),
                        Ok(matches) => {
                            if config.count {
                                print(&filename, &format!("{}\n", matches.len()));
                            } else {
                                for line in &matches {
                                        print(&filename, line);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn find_files(paths: &[String], recursive: bool) -> Vec<MyResult<String>> {
    let mut results = vec![];

    for path in paths {
        match path.as_str() {
            "-" => results.push(Ok(path.to_string())),
            _ => match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                          for entry in WalkDir::new(path)
                              .into_iter()
                              .flatten()
                              .filter(|e| e.file_type().is_file())
                          {
                              results.push(Ok(entry
                                  .path()
                                  .display()
                                  .to_string()));
                          }
                        } else {
                            results.push(Err(From::from(format!("{} is a directory", path))));
                        }
                    } else if metadata.is_file() {
                        results.push(Ok(path.to_string()));
                    }
                }
                Err(e) => {
                    results.push(Err(From::from(format!("{}: {} ", path, e))));
                }
            }
        }
    }

    results
}

fn find_lines<T: BufRead>(mut file: T, pattern: &Regex, invert_match: bool) -> MyResult<Vec<String>> {
    let mut matches = vec![];
    let mut lines = String::new();

    loop {
        let bytes = file.read_line(&mut lines)?;
        if bytes == 0 {
            break;
        }
        if pattern.is_match(&lines) ^ invert_match {
            matches.push(mem::take(&mut lines));
        }
        lines.clear();
    }
    Ok(matches)
}
fn _find_lines<T: BufRead>(mut file: T, pattern: &Regex, invert_match: bool) -> MyResult<Vec<String>> {
    let mut it = file.lines().peekable();
    let mut matches = vec![];
    while let Some(line) = it.next() {
        let is_last = it.peek().is_none();

        let line = line?;
        // Do not need this break - we take every line that is read from file
        // if line.len() == 0 {
        //     break;
        // }

        if pattern.is_match(&line) ^ invert_match {
            matches.push(line);
        }
    }

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distr::Alphanumeric, Rng};

    #[test]
    fn test_find_files() {
        let files = find_files(&[String::from("tests/inputs/fox.txt")], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "tests/inputs/fox.txt");
    }

    #[test]
    fn test_find_lines() {

    }
}