use super::constants::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use das_types::{packed::*, prelude::*};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::{env, io, path::PathBuf};

pub fn read_lines(file_name: &str) -> Result<Lines<BufReader<File>>> {
    let dir = env::current_dir().unwrap();
    let mut file_path = PathBuf::new();
    file_path.push(dir);
    file_path.push("data");
    file_path.push(file_name);

    // Read record keys from file, then sort them.
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn gen_price_config(length: u8, new_price: u64, renew_price: u64) -> PriceConfig {
    PriceConfig::new_builder()
        .length(Uint8::from(length))
        .new(Uint64::from(new_price))
        .renew(Uint64::from(renew_price))
        .build()
}

pub fn prepend_molecule_like_length(raw: Vec<u8>) -> Vec<u8> {
    if raw.len() + 4 > WITNESS_SIZE_LIMIT {
        panic!("\nWitness reached the size limit which is 32KB.")
    }

    // Prepend length of bytes to raw data, include the bytes of length itself.
    let mut entity = (raw.len() as u32 + 4).to_le_bytes().to_vec();
    entity.extend(raw);

    entity
}

pub fn gen_timestamp(datetime: &str) -> u64 {
    let navie_datetime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S")
        .expect("Invalid datetime format.");
    let datetime = DateTime::<Utc>::from_utc(navie_datetime, Utc);
    datetime.timestamp() as u64
}
