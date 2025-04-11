//use std::fs::File;
use csv::ReaderBuilder;
use std::error::Error;

pub fn read_csv(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        data.push(record.iter().map(|s| s.to_string()).collect());
    }
    Ok(data)
}
