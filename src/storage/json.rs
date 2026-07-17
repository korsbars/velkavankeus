use crate::modeling::portfolio::Portfolio;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind};

pub fn save_portfolio(
    portfolio: &Portfolio,
    path: &str,
) -> Result<(), Error> {
    // Open the file for writing (creates or overwrites)
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    // Serialize to pretty-printed JSON and write to file
    serde_json::to_writer_pretty(writer, portfolio)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(())
}

pub fn load_portfolio(
    path: &str,
) -> Result<Portfolio, Error> {
    // Open the file for reading
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON back into a Portfolio instance
    let portfolio: Portfolio = serde_json::from_reader(reader)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    Ok(portfolio)
}
