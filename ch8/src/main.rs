fn main() {
    if let Err(e) = ch8_cutr::get_args().and_then(ch8_cutr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
// fn main() -> std::io::Result<()> {
//     let mut reader = ReaderBuilder::new()
//         .delimiter(b',')
//         .from_reader(File::open("ch8/tests/inputs/books.csv")?);
//
//     println!("{}", fmt(reader.headers()?));
//     for record in reader.records() {
//         println!("{}", fmt(&record?));
//     }
//
//     Ok(())
// }

// fn fmt(rec: &StringRecord) -> String {
//     rec.into_iter().map(|v| format!("{:20}", v)).collect()
// }