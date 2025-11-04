fn main() {
    if let Err(e) = ch12_fortuner::get_args().and_then(ch12_fortuner::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
