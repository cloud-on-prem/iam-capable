use std::str::FromStr;
use std::io::Write;

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

pub fn print<T: OutputSerializable, W: Write>(
    output_format: OutputFormat,
    data: &[T],
    writer: &mut W,
) {
    match output_format {
        OutputFormat::Csv => {
            // Output as CSV
            if let Err(e) = write_csv(data, writer) {
                eprintln!("Error writing CSV: {}", e);
            }
        }
        OutputFormat::Json => {
            // Output as JSON
            if let Err(e) = write_json(data, writer) {
                eprintln!("Error writing JSON: {}", e);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::{OutputSerializable, print, OutputFormat};

    // Dummy struct for testing purposes
    #[derive(Debug, PartialEq)]
    struct TestData {
        value1: String,
        value2: i32,
    }

    impl OutputSerializable for TestData {
        fn csv_header() -> Vec<&'static str> {
            vec!["value1", "value2"]
        }

        fn csv_record(&self) -> Vec<String> {
            vec![self.value1.clone(), self.value2.to_string()]
        }

        fn to_json_value(&self) -> serde_json::Value {
            serde_json::json!({
                "value1": self.value1,
                "value2": self.value2,
            })
        }
    }

    #[test]
    fn test_print_csv() {
        let data = vec![
            TestData {
                value1: "test1".to_string(),
                value2: 42,
            },
            TestData {
                value1: "test2".to_string(),
                value2: 99,
            },
        ];

        let mut cursor = Cursor::new(Vec::new());
        print(OutputFormat::Csv, &data, &mut cursor);

        let csv_output = String::from_utf8(cursor.into_inner()).unwrap();
        assert_eq!(csv_output, "value1,value2\ntest1,42\ntest2,99\n");
    }

    #[test]
    fn test_print_json() {
        let data = vec![
            TestData {
                value1: "test1".to_string(),
                value2: 42,
            },
            TestData {
                value1: "test2".to_string(),
                value2: 99,
            },
        ];

        let mut cursor = Cursor::new(Vec::new());
        print(OutputFormat::Json, &data, &mut cursor);

        let json_output = String::from_utf8(cursor.into_inner()).unwrap();
        let parsed_json: serde_json::Value = serde_json::from_str(&json_output).unwrap();
        let expected_json: serde_json::Value = serde_json::json!([
            {"value1": "test1", "value2": 42},
            {"value1": "test2", "value2": 99},
        ]);

        assert_eq!(parsed_json, expected_json);
    }

}
