use crate::compare::CapabilityRow;
use serde_json::json;
use std::io::Write;

pub fn write_json<W: Write>(rows: &[CapabilityRow], writer: W) -> Result<(), serde_json::Error> {
    let json_rows = rows
        .iter()
        .map(|row| {
            json!({
                "resource": row.resource,
                "action": row.action,
                "role1": row.has_capability1,
                "role2": row.has_capability2
            })
        })
        .collect::<Vec<_>>();

    serde_json::to_writer(writer, &json_rows)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
