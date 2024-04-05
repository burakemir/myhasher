use std::io::prelude::BufRead;
use std::io::Result;

use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct FileWithHash {
    pub filename: String,
    pub hash: [u8; 32],
}

//
pub fn hash(buf_reader: &mut dyn BufRead, filename: &str) -> Result<FileWithHash> {
    let mut hasher = Sha256::new();
    std::io::copy(buf_reader, &mut hasher)?;
    Ok(FileWithHash {
        filename: filename.to_string(),
        hash: hasher.finalize().into(),
    })
}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    #[test]
    fn test_hash() -> Result<()> {
        let str = "Hello, world!\n";
        let mut bytes = str.as_bytes();
        let file_with_hash = super::hash(&mut bytes, "myfile");
        verify_that!(file_with_hash.is_ok(), eq(true))?;
        let hash = file_with_hash.unwrap().hash.to_vec();
        let expected =
            hex::decode("d9014c4624844aa5bac314773d6b689ad467fa4e1d1a50a1b8a99d5a95f72ff5")
                .unwrap();
        verify_that!(hash, pointwise!(eq, expected))?;
        Ok(())
    }
}
