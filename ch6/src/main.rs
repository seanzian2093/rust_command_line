fn main() {
    if let Err(e) = ch6_uniqr::get_args().and_then(ch6_uniqr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
