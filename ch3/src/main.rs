fn main() {
    // Use function defined in lib.rs
    // get_args returns a Ok(config) if successful, and pass it to run as argument
    if let Err(e) = ch3_catr::get_args().and_then(ch3_catr::_run) {
        // if let Err(e) = ch3_catr::get_args().and_then(ch3_catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
