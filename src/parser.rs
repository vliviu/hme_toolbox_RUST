/////
// src/parser.rs
use std::fs::File;
//use std::io::{BufReader, Read};

use std::io::{BufReader};  // Remove `Read` if you're not using it
use bio::io::fasta::Reader;


pub fn parse_fasta(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let records = Reader::new(reader)
        .records()
        .filter_map(Result::ok)
        .map(|rec| {
            // Convert Vec<u8> to String
            String::from_utf8(rec.seq().to_owned()).unwrap_or_else(|_| String::from("Invalid"))
        })
        .collect(); // Now collects Vec<String>

    Ok(records)
}
