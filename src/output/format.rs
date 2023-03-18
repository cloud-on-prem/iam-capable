use std::str::FromStr;
use std::io::stdout;

use super::csv::{write_csv};
use super::json::write_json;

#[derive(Debug, PartialEq)]
pub enum OutputFormat {
    Csv,
    Json,
}

pub trait OutputSerializable {
    fn csv_header() -> Vec<&'static str>;
    fn csv_record(&self) -> Vec<String>;
    fn to_json_value(&self) -> serde_json::Value;
}


impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(OutputFormat::Csv),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}

pub fn print<T: OutputSerializable>(output_format: OutputFormat, data: &[T]) {
    match output_format {
        OutputFormat::Csv => {
            // Output as CSV
            if let Err(e) = write_csv(data, stdout()) {
                eprintln!("Error writing CSV: {}", e);
            }
        }
        OutputFormat::Json => {
            // Output as JSON
            if let Err(e) = write_json(data, stdout()) {
                eprintln!("Error writing JSON: {}", e);
            }
        }
    }
}
