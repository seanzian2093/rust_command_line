fn main() {
    if let Err(e) = ch11_tailr::get_args().and_then(ch11_tailr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
