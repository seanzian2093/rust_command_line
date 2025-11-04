use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
};
use std::ffi::OsStr;
use std::path::PathBuf;
use clap::{Arg, ArgAction, Command};
use rand::prelude::IndexedRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

#[derive(Debug)]
pub struct Fortune {
    source: String,
    text: String,
}

fn parse_u64(s: &str) -> MyResult<u64> {
    s.parse().map_err(|_| format!("\"{}\" not a valid integer", s).into())
}

fn find_files(paths: &[String]) -> MyResult<Vec<PathBuf>> {
    let dat = OsStr::new("dat");
    let mut files = vec![];

    for path in paths {
        match fs::metadata(path) {
            Err(e) => return Err(format!("{}: {}", path, e).into()),
            Ok(_) => files.extend(
                WalkDir::new(path)
                .into_iter()
                .filter_map(Result::ok)
                    .filter(|e| {
                        e.file_type().is_file() && e.path().extension() != Some(dat)
                    })
                    .map(|e| e.path().into())
            ),
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn read_fortunes(paths: &[PathBuf]) -> MyResult<Vec<Fortune>> {
    let mut fortunes = vec![];
    let mut buffer= vec![];

    for path in paths {
        let basename = path.file_name().unwrap().to_string_lossy().into_owned();
        let file = File::open(path).map_err(|e| { format!("{}: {}", path.to_string_lossy(), e) })?;

        for line in BufReader::new(file).lines().filter_map(Result::ok) {
            if line == "%" {
                if !buffer.is_empty() {
                    fortunes.push(Fortune {
                        source: basename.clone(),
                        text: buffer.join("\n"),
                    });
                    buffer.clear();
                }
            } else {
                buffer.push(line);
            }
        }
    }

    Ok(fortunes)
}

fn pick_fortunes(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    if let Some(val) = seed {
        let mut rng = StdRng::seed_from_u64(val);
        fortunes.choose(&mut rng).map(|f| f.text.to_string())
    } else {
        let mut rng = rand::rng();
        fortunes.choose(&mut rng).map(|f| f.text.to_string())
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch12-fortuner")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust fortune")
        .arg(
            Arg::new("sources")
                .value_name("FILE")
                .help("Input files or directories")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("pattern")
                .short('m')
                .long("pattern")
                .value_name("PATTERN")
                .help("Pattern"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive search")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .value_name("SEED")
                .help("Random seed"),
        )
        .get_matches();

    let sources= matches.get_many::<String>("sources")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();

    let pattern = matches
        .get_one::<String>("pattern")
        .map(|val| {
            RegexBuilder::new(val)
                .case_insensitive(matches.get_flag("insensitive"))
                .build()
                .map_err(|_| format!("Invalid --pattern \"{}\"", val))
        })
        .transpose()?;

    let seed = matches
        .get_one::<String>("seed")
        .map(|s|parse_u64(s))
        .transpose()?;

    Ok(Config {
        sources,
        pattern,
        seed,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.sources)?;
    let fortunes = read_fortunes(&files)?;
    if let Some(pattern) = config.pattern {
        let mut prev_source = None;
        for fortune in fortunes.iter().filter(|f| pattern.is_match(&f.text)) {
            if prev_source.as_ref().map_or(true, |s| s != &fortune.source) {
                eprintln!("({})\n%", fortune.source);
                prev_source = Some(fortune.source.clone());
            }
            println!("{}\n%", fortune.text);
        }
    } else {
        println!("{}", pick_fortunes(&fortunes, config.seed).or_else(|| Some("No fortunes found".to_string())).unwrap());

    }
    Ok(())
}