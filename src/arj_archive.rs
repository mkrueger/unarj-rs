use std::io::{self, Read, Seek};

use delharc::decode::{Decoder, DecoderAny};

use crate::{local_file_header::{CompressionMethod, LocalFileHeader}, main_header::MainHeader};

pub struct ArjArchieve<T: Read + Seek> {
    reader: T,
    pub header: MainHeader,
}

impl<T: Read + Seek> ArjArchieve<T> {
    pub fn new(mut reader: T) -> io::Result<Self> {
        let header_bytes = read_header(&mut reader)?;
        if header_bytes.is_empty() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Archive ends without any headers"));
        }
        let header = MainHeader::load_from(&header_bytes);
        // Skip extended headers
        read_extended_headers(&mut reader)?;

        Ok(Self {
            header,
            reader
        })
    }
     
    pub fn skip(&mut self, header: &LocalFileHeader) -> io::Result<()> {
        self.reader.seek( io::SeekFrom::Current(header.compressed_size as i64))?;
        Ok(())        
    }

    pub fn read(&mut self, header: &LocalFileHeader) -> io::Result<Vec<u8>> {
        let mut compressed_buffer = vec![0; header.compressed_size as usize];
        self.reader.read_exact(&mut compressed_buffer)?;

        match header.compression_method {
            CompressionMethod::Stored => Ok(compressed_buffer),
            CompressionMethod::CompressedMost |
            CompressionMethod::Compressed |
            CompressionMethod::CompressedFaster => {
                let mut decoder = DecoderAny::new_from_compression(delharc::CompressionMethod::Lh6,compressed_buffer.as_slice());
                let mut decompressed_buffer = vec![0; header.original_size as usize];
                decoder.fill_buffer(&mut decompressed_buffer)?;
                Ok(decompressed_buffer)
            }
            CompressionMethod::CompressedFastest =>  Err(io::Error::new(io::ErrorKind::InvalidData, "Compression method 4 not implemented")),
            CompressionMethod::NoDataNoCrc |
            CompressionMethod::NoData |
            CompressionMethod::Unknown(_) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unsupported compression method {:?}", header.compression_method)))
        }
    }

    pub fn get_next_entry(&mut self) -> io::Result<Option<LocalFileHeader>> {
        let header_bytes = read_header(&mut self.reader)?;
        if header_bytes.is_empty() {
            return Ok(None);
        }
        let current_local_file_header = LocalFileHeader::load_from(&header_bytes);
        if current_local_file_header.is_none() {
            return Ok(None);
        }
        read_extended_headers(&mut self.reader)?;
        Ok(current_local_file_header)
    }

    
}

const MAX_HEADER_SIZE: usize = 2600;
const ARJ_MAGIC_1: u8 = 0x60;
const ARJ_MAGIC_2: u8 = 0xEA;

fn read_header<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut u8_buf = [0];
    loop {
        reader.read_exact(&mut u8_buf)?;
        if u8_buf[0] != ARJ_MAGIC_1 {
            println!("SKIP!!!");
            continue;
        }
        reader.read_exact(&mut u8_buf)?;
        if u8_buf[0] == ARJ_MAGIC_2 {
            break;
        }
    }
    let mut u16_buf = [0, 0];
    reader.read_exact(&mut u16_buf)?;

    let header_size = u16_buf[0] as u16 | (u16_buf[1] as u16) << 8;
    if header_size == 0 {
        return Ok(Vec::new());
    }
    if header_size > MAX_HEADER_SIZE as u16 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Header size is too big"));
    }
    let mut header_bytes = vec![0; header_size as usize];
    reader.read_exact(&mut header_bytes)?;
    let mut crc = [0, 0, 0, 0];
    reader.read_exact(&mut crc)?;
    let checksum = crc32fast::hash(&header_bytes);
    if checksum != u32::from_le_bytes(crc) {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Header checksum is invalid"))
    } else {
        Ok(header_bytes)
    }
}

fn read_extended_headers<R: Read>(reader: &mut R) -> io::Result<Vec<Vec<u8>>> {
    let mut extended_header =Vec::new();
    let mut u16_buf = [0, 0];
    loop {
        reader.read_exact(&mut u16_buf)?;
        let ext_header_size = u16_buf[0] as u16 | (u16_buf[1] as u16) << 8;
        if ext_header_size == 0 {
            return Ok(extended_header);
        }
        let mut header = vec![0; ext_header_size as usize];
        reader.read_exact(&mut header)?;
        let mut crc = [0, 0, 0, 0];
        reader.read_exact(&mut crc)?;
        let checksum = crc32fast::hash(&header);
        if checksum != u32::from_le_bytes(crc) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Extended header checksum is invalid"))
        }
        extended_header.push(header);
    }
}
