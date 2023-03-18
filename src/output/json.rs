use serde_json;
use std::io::Write;

use super::format::OutputSerializable;

pub fn write_json<T: OutputSerializable, W: Write>(rows: &[T], writer: W) -> Result<(), serde_json::Error> {
    let json_rows: Vec<serde_json::Value> = rows.iter().map(T::to_json_value).collect();
    serde_json::to_writer(writer, &json_rows)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::CapabilityRow;
    #[derive(serde::Deserialize)]
    struct JsonCapabilityRow {
        resource: String,
        action: String,
        role1: bool,
        role2: bool,
    }

    #[test]
    fn test_write_json() -> Result<(), Box<dyn std::error::Error>> {
        let rows = vec![
            CapabilityRow {
                resource: "arn:aws:s3:::mybucket".to_string(),
                action: "s3:ListBucket".to_string(),
                has_capability1: true,
                has_capability2: false,
            },
            CapabilityRow {
                resource: "arn:aws:s3:::mybucket/*".to_string(),
                action: "s3:GetObject".to_string(),
                has_capability1: false,
                has_capability2: true,
            },
        ];

        let mut buffer = Vec::new();
        write_json(&rows, &mut buffer)?;

        let deserialized_rows: Vec<JsonCapabilityRow> = serde_json::from_slice(&buffer)?;

        assert_eq!(deserialized_rows.len(), 2);

        assert_eq!(deserialized_rows[0].resource, "arn:aws:s3:::mybucket");
        assert_eq!(deserialized_rows[0].action, "s3:ListBucket");
        assert_eq!(deserialized_rows[0].role1, true);
        assert_eq!(deserialized_rows[0].role2, false);

        assert_eq!(deserialized_rows[1].resource, "arn:aws:s3:::mybucket/*");
        assert_eq!(deserialized_rows[1].action, "s3:GetObject");
        assert_eq!(deserialized_rows[1].role1, false);
        assert_eq!(deserialized_rows[1].role2, true);

        Ok(())
    }
}
