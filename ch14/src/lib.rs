use clap::{Arg, ArgAction, Command};
use std::{
    error::Error,
    fs,
    os::unix::fs::MetadataExt,
    path::PathBuf
};
use chrono::{DateTime, Local};
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    long: bool,
    show_hidden: bool,
}

#[derive(Clone, Copy)]
pub enum Owner {
    User,
    Group,
    Other,
}

impl Owner {
    pub fn masks(&self) -> [u32; 3] {
        match self {
            Self::User => [0o400, 0o200, 0o100],
            Self::Group => [0o040, 0o020, 0o010],
            Self::Other => [0o004, 0o002, 0o001],
        }
    }
}


pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch14-lsr")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust ls")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Files and/or directories")
                .num_args(1..)
                .default_value(".")
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("Long listing")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("all")
                .help("Show all files")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let paths= matches.get_many::<String>("paths")
        .unwrap()
        .cloned()
        .collect::<Vec<String>>();

    let long = matches.get_flag("long");
    let all = matches.get_flag("all");

    Ok(Config {
        paths,
        long,
        show_hidden: all,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let paths = find_files(&config.paths, config.show_hidden)?;
    if config.long {
        println!("{}", format_output(&paths)?);
    } else {
        for path in paths {
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> MyResult<Vec<PathBuf>> {
    let mut results = vec![];
    for name in paths {
        match fs::metadata(name) {
            Err(e) => eprintln!("{}: {}", name, e),
            Ok(meta) => {
                if meta.is_dir() {
                    for entry in fs::read_dir(name)? {
                        let entry = entry?;
                        let path = entry.path();
                        let is_hidden = path.file_name().map_or(false, |x| x.to_string_lossy().starts_with("."));
                        if !is_hidden || show_hidden {
                            results.push(entry.path());
                        }
                    }
                } else {
                    results.push(PathBuf::from(name));
                }
            }

        }
    }
    Ok(results)
}

fn format_output(paths: &[PathBuf]) -> MyResult<String> {
    let fmt = "{:<}{:<} {:>} {:<} {:<} {:>} {:<} {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let metadata = path.metadata()?;

        let uid = metadata.uid();
        let user = get_user_by_uid(uid)
            .map(|u| u.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| uid.to_string());

        let gid = metadata.gid();
        let group = get_group_by_gid(gid)
            .map(|g| g.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| gid.to_string());

        let file_type = if path.is_dir() { "d" } else { "-" };
        let perms = format_mode(metadata.mode());
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);


        table.add_row(
            Row::new()
                .with_cell(file_type)
                .with_cell(perms)
                .with_cell(metadata.nlink())
                .with_cell(user)
                .with_cell(group)
                .with_cell(metadata.len())
                .with_cell(modified.format("%b %d %y %H:%M"))
                .with_cell(path.display())
        );
    }
    Ok(format!("{}", table))
}

// Given a file mode in octal format like 0o751
// return a string like "rwxr-x--x"
fn format_mode(mode: u32) -> String {
    format!(
        "{}{}{}",
        mk_triple(mode, Owner::User),
        mk_triple(mode, Owner::Group),
        mk_triple(mode, Owner::Other),
    )
}
// Given an octal format like 0o751 and an ['Owner'],
// return a string like "r-x"
pub fn mk_triple(mode: u32, owner: Owner) -> String {
    let [read, write, execute] = owner.masks();
    format!(
        "{}{}{}",
        if mode & read == 0 { "-" } else { "r" },
        if mode & write == 0 { "-" } else { "w" },
        if mode & execute == 0 { "-" } else { "x" },
    )
}
