import::import! {
    "//third_party/hex";
    "//third_party/sha2";
    "//third_party/clap";
}

mod myhasher;

use clap::Parser;
use myhasher::hash;
use std::fs::read_dir;
use std::fs::File;
use std::io;
use std::io::{BufReader, Result};
use std::path::Path;

/// Simple program to hash input from stin, a file, or a dir.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Files to fingerprint
    #[clap(value_parser)]
    names: Vec<String>,

    /// Directory to fingerprint
    #[clap(short, long, value_parser)]
    dir: Option<String>,
}

fn hash_file(file: &File, name: String) -> Result<()> {
    let mut buf_reader = BufReader::new(file);
    let work = hash(&mut buf_reader, name)?;
    println!("{} {}", hex::encode(work.hash), work.filename);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.names.is_empty() && args.dir.is_none() {
        let work = hash(&mut io::stdin().lock(), "".to_string())?;
        println!("{} stdin", hex::encode(work.hash));
        return Ok(());
    }

    for arg in args.names {
        let file = File::open(arg.as_str())?;
        hash_file(&file, arg)?
    }

    if let Some(dir) = args.dir.as_deref() {
        if !Path::new(dir).is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                std::format!("not a dir {}", dir),
            ));
        }
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file = File::open(&path)?;
                hash_file(&file, path.to_string_lossy().to_string())?
            }
        }
    }
    Ok(())
}
