fn main() {
    if let Err(e) = ch9_grepr::get_args().and_then(ch9_grepr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
