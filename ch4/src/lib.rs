use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

fn parse_positive_int(s: &str) -> MyResult<usize> {
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(s)),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch4-headr")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust head")
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .conflicts_with("lines")
        )
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                // .multiple(true)
                .num_args(1..)
                .default_value("-"),
        )
        .get_matches();

    let files = matches.get_many::<String>("files")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();

    let lines = matches.get_many::<String>("lines")
        .unwrap()
        .cloned()
        .collect::<String>();
    let lines = parse_positive_int(lines.as_str()).map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches.get_one::<String>("bytes")
        .map(|s| s.as_str())
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut file) => {
                // file separators
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                // -c flag in turned on
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    // otherwise
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        // read_line() reads all bytes until a new line is reached
                        // appends bytes read to provided buffer
                        // return size of bytes read as usize
                        let bytes = file.read_line(&mut line)?;
                        // if EOF, bytes read is 0, stop reading; for blank line bytes read is 1
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        // print!("{}: {}", line, bytes);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}
