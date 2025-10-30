fn main() {
    if let Err(e) = ch5_wcr::get_args().and_then(ch5_wcr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
