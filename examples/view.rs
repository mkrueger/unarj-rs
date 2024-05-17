use std::fs;
use unarj_rs::arj_archive::ArjArchieve;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut archieve = ArjArchieve::new(fs::File::open(path).unwrap()).unwrap();

    println!("Name            Size            Compression");
    println!("---------------------------------------------------");
    while let Ok(Some(header)) = archieve.get_next_entry() {
        println!(
            "{:<15}\t{:<7}\t\t{:?}",
            header.name, header.original_size, header.compression_method
        );
        archieve.skip(&header).unwrap();
    }
}
