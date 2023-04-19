use std::io::Result;
use std::io::prelude::BufRead;
use sha2::{Sha256, Digest};

#[derive(Clone)]
pub struct FileWithHash {
    pub filename: String,
    pub hash: [u8; 32],
}

pub fn hash(buf_reader: &mut dyn BufRead, filename: String) -> Result<FileWithHash> {
    let mut hasher = Sha256::new(); 
    std::io::copy(buf_reader, &mut hasher)?;
    return Ok(FileWithHash {
        filename: filename,
        hash: hasher.finalize().into(),
    });
}