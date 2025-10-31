use clap::{Arg, ArgAction, Command};

// target/debug/ch2-echor -h
fn main() {
    let matches = Command::new("ch2-echor")
        .version("0.1.0")
        .author("Sean Z")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..)
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    // let text = matches.get_one::<String>("text").expect("required argument `text` is missing");
    let text = matches.get_many::<String>("text")
                                              .unwrap()
                                              .cloned()
                                              .collect::<Vec<String>>()
        .join(" ");
    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    // println!() always appends its own new line. So use print! instead
    // println!("{}{}", text, ending);
    print!("{}{}", text, ending);
}
