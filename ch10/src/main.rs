fn main() {
    if let Err(e) = ch10_commr::get_args().and_then(ch10_commr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
