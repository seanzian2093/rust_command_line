fn main() {
    if let Err(e) = ch13_calr::get_args().and_then(ch13_calr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
