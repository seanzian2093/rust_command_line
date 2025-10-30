
fn main() {
    if let Err(e) = ch4_headr::get_args().and_then(ch4_headr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
