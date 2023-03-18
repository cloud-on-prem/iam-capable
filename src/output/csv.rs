use super::format::OutputSerializable;
use csv::Writer;
use std::io::Write;

pub fn write_csv<T: OutputSerializable, W: Write>(rows: &[T], writer: W) -> Result<(), csv::Error> {
    let mut csv_writer = Writer::from_writer(writer);

    // Write the header
    csv_writer.write_record(T::csv_header())?;

    // Write rows
    for row in rows {
        csv_writer.write_record(&row.csv_record())?;
    }

    csv_writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::super::capability::CapabilityComparisonRow;
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write_csv_empty_rows() {
        let rows: Vec<CapabilityComparisonRow> = Vec::new();
        let mut writer = Cursor::new(Vec::new());

        assert!(write_csv(&rows, &mut writer).is_ok());

        let content = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(content, "Resource,Action,Role1,Role2\n");
    }

    // Test case: single row
    #[test]
    fn test_write_csv_single_row() {
        let rows = vec![CapabilityComparisonRow {
            resource: String::from("Resource1"),
            action: String::from("Action1"),
            has_capability1: true,
            has_capability2: false,
        }];
        let mut writer = Cursor::new(Vec::new());

        assert!(write_csv(&rows, &mut writer).is_ok());

        let content = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(
            content,
            "Resource,Action,Role1,Role2\nResource1,Action1,true,false\n"
        );
    }

    // Test case: multiple rows
    #[test]
    fn test_write_csv_multiple_rows() {
        let rows = vec![
            CapabilityComparisonRow {
                resource: String::from("Resource1"),
                action: String::from("Action1"),
                has_capability1: true,
                has_capability2: false,
            },
            CapabilityComparisonRow {
                resource: String::from("Resource2"),
                action: String::from("Action2"),
                has_capability1: false,
                has_capability2: true,
            },
        ];
        let mut writer = Cursor::new(Vec::new());

        assert!(write_csv(&rows, &mut writer).is_ok());

        let content = String::from_utf8(writer.into_inner()).unwrap();
        assert_eq!(
        content,
        "Resource,Action,Role1,Role2\nResource1,Action1,true,false\nResource2,Action2,false,true\n"
    );
    }
}
