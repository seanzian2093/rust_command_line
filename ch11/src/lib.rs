use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek},
};
use std::io::SeekFrom;
use clap::{Arg, ArgAction, Command};
use regex::Regex;
use once_cell::sync::OnceCell;

static NUM_RE: OnceCell<Regex> = OnceCell::new();

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
}

fn count_lines_bytes(filename: &str) -> MyResult<(i64, i64)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut buf = Vec::new();

    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }

        num_lines += 1;
        num_bytes += bytes_read as i64;
        buf.clear();
    }
    Ok((num_lines, num_bytes))
}

fn print_lines(
    mut file: impl BufRead,
    num_lines: &TakeValue,
    total_lines: i64,
) -> MyResult<()> {
    if let Some(start) = get_start_index(num_lines, total_lines) {
        let mut line_num = 0;
        let mut buf = Vec::new();

        loop {
            let bytes_read = file.read_until(b'\n', &mut buf)?;
            if bytes_read == 0 {
                break;
            }

            if line_num >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }

            line_num += 1;
            buf.clear();
        }
    }
    Ok(())
}

fn get_start_index(
    take_val: &TakeValue,
    total: i64,
) -> Option<u64> {
    match take_val {
        TakeValue::PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        }
        TakeValue::TakeNum(num) => {
            if num == &0 || total == 0 || num > &total {
                None
            } else {
                let start = if num < &0 { total + num } else { num - 1};
                Some(if start < 0 { 0 } else { start as u64})
            }
        }
    }
}

fn print_bytes<T: Read + Seek>(
    mut file: T,
    num_bytes: &TakeValue,
    total_bytes: i64,
) -> MyResult<()> {
    if let Some(start) = get_start_index(num_bytes, total_bytes) {
        file.seek(SeekFrom::Start(start))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        if !buf.is_empty() {
            print!("{}", String::from_utf8_lossy(&buf));
        }
    }
    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match File::open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if !config.quiet && num_files > 1 {
                   println!(
                       "{}==> {} <==",
                       if file_num > 0 { "\n" } else { "" },
                       filename
                   );
                }

                let (total_lines, total_bytes) = count_lines_bytes(&filename)?;
                let file = BufReader::new(file);
                if let Some(num_bytes) = &config.bytes {
                    print_bytes(file, num_bytes, total_bytes)?;
                } else {
                    print_lines(file, &config.lines, total_lines)?;
                }
            }
        }
    }
    Ok(())
}

fn parse_num(val: &str) -> MyResult<TakeValue> {
    // below will init a Regex for every call of parse_num
    // let num_re = Regex::new(r"^([+-])?(\d+)$").unwrap();
    // use once_cell
    let num_re = NUM_RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());

    match num_re.captures(val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("-", |m| m.as_str());
            let num = format!("{}{}", sign, caps.get(2).unwrap().as_str());
            if let Ok(val) = num.parse() {
                if sign == "-" && val == 0 {
                    Ok(TakeValue::PlusZero)
                } else {
                    Ok(TakeValue::TakeNum(val))
                }
            } else {
                Err(From::from(val))
            }
        }
        _ => Err(From::from(val)),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch11-tailr")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust tail")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s) ")
                .num_args(1..)
                .required(true)
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10")
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("Number of bytes")
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress headers")
        )
        .get_matches();

    let lines = matches
        .get_one::<String>("lines")
        .map(|s| parse_num(&s))
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?
        .unwrap();

    let files = matches.get_many::<String>("files")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();

    let bytes= matches
        .get_one::<String>("bytes")
        .map(|s| parse_num(&s))
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    let quiet = matches.get_flag("quiet");

    Ok(Config {
        files,
        lines,
        bytes,
        quiet,
    })
}