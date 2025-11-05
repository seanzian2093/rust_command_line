use clap::{Arg, ArgAction, Command};
use std::{
    error::Error,
    str::FromStr,
};
use chrono::{NaiveDate, Datelike, Local};
use ansi_term::Style;
use itertools::{izip, Itertools};

type MyResult<T> = Result<T, Box<dyn Error>>;

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "Novmember",
    "December",
];

const LINE_WIDTH: usize = 22;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("ch13-calr")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust cal")
        .arg(
            Arg::new("month")
                .value_name("MONTH")
                .short('m')
                .help("Month name or number (1-12)")
                .num_args(1)
        )
        .arg(
            Arg::new("show_current_year")
                .short('y')
                .long("year")
                .value_name("SHOW_YEAR")
                .help("Show whole current year")
                .conflicts_with_all(&["month", "year"])
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("year")
                .value_name("YEAR")
                .help("Year (1-9999)")
        )
        .get_matches();

    let mut month = matches.get_one::<String>("month")
        .map(|s| parse_month(s))
        .transpose()?;

    let mut year = matches.get_one::<String>("year")
        .map(|s| parse_year(s))
        .transpose()?;

    let today = Local::now();
    if matches.get_flag("show_current_year") {
        month = None;
        year = Some(today.year());
    } else if month.is_none() && year.is_none() {
        month = Some(today.month());
        year = Some(today.year());
    }

    Ok(Config {
        month,
        year: year.unwrap_or_else(|| today.year()),
        today: today.date_naive(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    match config.month {
        Some(month) => {
            let lines = format_month(config.year, month, true, config.today);
            println!("{}", lines.join("\n"));
        }
        None => {
            println!("{:>32}", config.year);
            let months: Vec<_> = (1..=12).into_iter().map(|m| {
                format_month(config.year, m, false, config.today)
            }).collect();

            for (i, chunk) in months.chunks(3).enumerate() {
                if let [m1, m2, m3] = chunk {
                    for lines in izip!(m1, m2, m3) {
                        println!("{}{}{}", lines.0, lines.1, lines.2);
                    }
                    if i < 3 { println!(); }
                }
            }
        }
    }
    Ok(())
}

fn parse_int<T: FromStr>(s: &str) -> MyResult<T> {
    s.parse().map_err(|_| format!("Invalid integer: \"{}\"", s).into())
}

fn parse_year(year: &str) -> MyResult<i32> {
    parse_int(year).and_then(|num| {
        if (1..=9999).contains(&num) {
            Ok(num)
        } else {
            Err(format!("year \"{}\" not in the range 1 through 9999", year).into())
        }
    })
}

fn parse_month(month: &str) -> MyResult<u32> {
    match parse_int(month) {
        Ok(num) => {
            if (1..=12).contains(&num) {
                Ok(num)
            } else {
                Err(format!("month \"{}\" not in the range 1 through 12", month).into())
            }
        }

        _ => {
            let lower = &month.to_lowercase();
            let matches: Vec<_> = MONTH_NAMES
                .iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    if name.to_lowercase().starts_with(lower) {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect();
            if matches.len() == 1 {
                Ok(matches[0] as u32)
            } else {
                Err(format!("Invalid month \"{}\"", month).into())
            }
        }
    }
}

fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    NaiveDate::from_ymd_opt(y, m, 1).unwrap()
}

fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let mut days: Vec<String> = (1..first.weekday().number_from_sunday())
        .into_iter()
        .map(|_| "  ".to_string())
        .collect();

    let is_today = |day: u32| {
        year == today.year() && month == today.month() && day == today.day()
    };

    let last = last_day_in_month(year, month);
    days.extend((first.day()..=last.day()).into_iter().map(|num| {
        let fmt = format!("{:>2}", num);
        if is_today(num) {
            Style::new().reverse().paint(fmt).to_string()
        } else { fmt }

    }));

    let month_name = MONTH_NAMES[month as usize -1];
    let mut lines = Vec::with_capacity(8);
    lines.push(format!(
        "{:^20}  ",
        if print_year { format!("{} {}", month_name, year)} else { month_name.to_string() }
    ));

    lines.push("Su Mo Tu We Th Fr Sa  ".to_string());

    for week in days.chunks(7) {
        lines.push(format!("{:width$}  ", week.join(" "), width = LINE_WIDTH - 2));
    }

    while lines.len() < 8 {
        lines.push(" ".repeat(LINE_WIDTH));
    }

    lines
}

