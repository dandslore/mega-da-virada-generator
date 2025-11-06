use anyhow::{Context, Result};
use chrono::Utc;
use rand::seq::IteratorRandom;
use rusqlite::{Connection, OptionalExtension, params};
use sha3::{Digest, Sha3_256};
use std::io::{BufReader, Read};
use std::path::Path;
use std::{fs, num};

pub fn sha3_256_of_file(path: &Path) -> Result<String> {
    let file = fs::File::open(path)
        .with_context(|| format!("Falha ao abrir arquivo para checksum: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha3_256::new();
    let mut buffer = [0u8; 4096];

    loop {
        let read_count = reader.read(&mut buffer)?;
        if read_count == 0 {
            break;
        }
        hasher.update(&buffer[..read_count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
