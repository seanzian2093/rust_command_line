fn main() {
    if let Err(e) = ch14_lsr::get_args().and_then(ch14_lsr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
