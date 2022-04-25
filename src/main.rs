
use std::fs::File;
use std::io;
use std::io::{BufReader, Result};
use std::io::prelude::BufRead;

use sha2::{Sha256, Digest};
use std::env;
use generic_array::GenericArray;
use generic_array::typenum::U32;

#[derive(Clone)]
struct FileWithHash {
    filename: String,
    hash: GenericArray<u8, U32>,
}

fn hash(buf_reader: &mut dyn BufRead, filename: String) -> Result<FileWithHash> {
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

fn main() -> Result<()> {
    if env::args().len() == 1 {
        let work = hash(&mut io::stdin().lock(), "".to_string())?;
        println!("{:x} stdin", work.hash);
    } else {
        for arg in env::args().skip(1) {
            let mut buf_reader = BufReader::new(File::open(&arg)?);
            let work = hash(&mut buf_reader, arg)?;
            println!("{:x} {}", work.hash, work.filename);
        }
    }
    Ok(())
}
