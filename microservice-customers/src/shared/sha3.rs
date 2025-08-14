use std::{fs::File, io::{BufReader, Read}};
use sha3::{Sha3_256, Digest};

pub fn sha3_256_of_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 4096];
    let mut hasher = Sha3_256::new();

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}