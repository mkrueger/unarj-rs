use std::io::Cursor;
use unarj_rs::arj_archive::ArjArchieve;

#[test]
fn wrong_crc32() {
    let file = Cursor::new(include_bytes!("data/wrongcrc32.arj"));
    let mut archieve = ArjArchieve::new(file).unwrap();
    let entry = archieve.get_next_entry().unwrap().unwrap();
    let result = archieve.read(&entry);
    assert!(result.is_err());
}
