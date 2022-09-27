
use std::io::Result;
use std::io::prelude::BufRead;

use sha2::{Sha256, Digest};
use generic_array::GenericArray;
use generic_array::typenum::U32;

#[derive(Clone)]
pub struct FileWithHash {
    pub filename: String,
    pub hash: GenericArray<u8, U32>,
}

pub fn hash(buf_reader: &mut dyn BufRead, filename: String) -> Result<FileWithHash> {
    let mut hasher = Sha256::new(); 

    loop {
        let buf = buf_reader.fill_buf()?;
        let len = buf.len();
        if len == 0 {
            break;
        }
        hasher.update(buf);
        buf_reader.consume(len);
    }

    return Ok(FileWithHash {
        filename: filename,
        hash: hasher.finalize(),
    });
}