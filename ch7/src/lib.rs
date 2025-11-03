#![allow(unused)]
use::clap::{Command, Arg};
use regex::Regex;
use std::error::Error;
use walkdir::{WalkDir, DirEntry};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}
#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch7-findr")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust find")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Search paths")
                // .multiple(true)
                .num_args(1..)
                .default_value("."),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n')
                .long("name")
                .help("NAME")
                .num_args(1..),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .help("Entry type")
                .value_parser(["f", "d", "l"])
                .num_args(1..),
        )
        .get_matches();

    let paths = matches.get_many::<String>("paths")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();

    let names = matches.get_many::<String>("names")

        .map(|vals| {
            vals.into_iter()
                .map(|name| {
                    Regex::new(&name)
                        .map_err(|_| format!("Invalid --name \"{}\"", name))
                })
                .collect::<Result<Vec<_>, _>>()

        })
        .transpose()
        .unwrap_or_default()
        .unwrap_or_default();

    let entry_types= matches.get_many::<String>("types")
        .map(|vals| {
            vals.into_iter()
                .map(|val| match val.as_str() {
                    "d" => EntryType::Dir,
                    "f" => EntryType::File,
                    "l" => EntryType::Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect::<Vec<EntryType>>()

        })
        .unwrap_or_default();


    Ok(Config { paths, names, entry_types})
}

pub fn run(config: Config) -> MyResult<()> {
    // Loop over each path
    for path in config.paths {
        // and over each entry in a path
        for entry in WalkDir::new(path) {
            // if entry is accessed successfully
            match entry {
                Err(e) => eprintln!("Error: {}", e),
                Ok(entry) => {
                    // and if entry_types in config is not specified or entry_types matches any of Dir/File/Link
                    if config.entry_types.is_empty()
                        || config.entry_types.iter().any(|entry_type| {
                        match entry_type {
                            EntryType::Dir => entry.file_type().is_dir(),
                            EntryType::File => entry.file_type().is_file(),
                            EntryType::Link => entry.file_type().is_symlink(),
                        }
                        // and if names/regex, are specified
                    }) && (config.names.is_empty() || config.names.iter().any(|re| {
                        re.is_match(&entry.file_name().to_string_lossy())
                    }))
                    // print entry's paths that matches a type or all types and match the name regex pattern
                    {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }
    Ok(())
}

// Refactor the run function using closure
pub fn _run(config: Config) -> MyResult<()> {
    // use walkdir::DirEntry, not std::fs::DirEntry
    let type_filter = |entry: &DirEntry| {
        let cond_1 = config.entry_types.is_empty();
        let cond_2 = config.entry_types.iter().any(|entry_type|
            match entry_type {
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
                EntryType::Link => entry.file_type().is_symlink(),
            }
        );

        cond_1 || cond_2
    };

    let name_filter = |entry: &DirEntry| {
        let cond_1 = config.names.is_empty();
        let cond_2 = config.names.iter().any(|re|
            re.is_match(&entry.file_name().to_string_lossy())
        );

        cond_1 || cond_2
    };
    
    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            // filter and map, only return those are Some(T)
        .filter_map(|e| match e {
            Err(e) => {
                eprintln!("Error: {}", e);
                None
            },
            Ok(entry) => Some(entry),
        })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        
        println!("{}", entries.join("\n"));
    }
    Ok(())
}
