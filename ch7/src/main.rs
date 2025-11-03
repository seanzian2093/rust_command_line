fn main() {
    // if let Err(e) = ch7_findr::get_args().and_then(ch7_findr::run) {
    if let Err(e) = ch7_findr::get_args().and_then(ch7_findr::_run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
