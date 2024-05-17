use std::io::Cursor;
use unarj_rs::{arj_archive::ArjArchieve, local_file_header::CompressionMethod};

#[test]
fn extract_method4() {
    let file = Cursor::new(include_bytes!("data/method4.arj"));
    let mut archieve = ArjArchieve::new(file).unwrap();
    assert_eq!("method4.arj", archieve.get_name());
    let entry = archieve.get_next_entry().unwrap().unwrap();
    assert_eq!("LICENSE", entry.name);
    assert_eq!(
        CompressionMethod::CompressedFastest,
        entry.compression_method
    );
    let result = archieve.read(&entry).unwrap();
    assert_eq!(include_bytes!("../LICENSE"), result.as_slice());
}
