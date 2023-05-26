import::import! {
    "//third_party/sha2";
}

use std::io::prelude::BufRead;
use std::io::Result;

use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct FileWithHash {
    pub filename: String,
    pub hash: [u8; 32],
}

pub fn hash(buf_reader: &mut dyn BufRead, filename: String) -> Result<FileWithHash> {
    let mut hasher = Sha256::new();
    std::io::copy(buf_reader, &mut hasher)?;
    Ok(FileWithHash {
        filename,
        hash: hasher.finalize().into(),
    })
}
