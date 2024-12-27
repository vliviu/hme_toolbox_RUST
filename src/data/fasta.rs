use bio::io::fasta;
//use std::fs::File;
use std::path::Path;

//pub fn read_fasta<P: AsRef<Path>>(path: P) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
pub fn read_fasta<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {

    let reader = fasta::Reader::from_file(path)?;
    let mut sequences = Vec::new();

    for record in reader.records() {
        let rec = record?;
        let id = rec.id().to_string();
        let seq = String::from_utf8(rec.seq().to_vec())?;
        sequences.push((id, seq));
    }
    
    Ok(sequences)
}
