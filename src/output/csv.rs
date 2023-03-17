use crate::compare::CapabilityRow;
use csv::Writer;
use std::io::Cursor;
use std::io::Write;

pub fn write_csv<W: Write>(rows: &[CapabilityRow], writer: W) -> Result<(), csv::Error> {
    let mut csv_writer = Writer::from_writer(writer);

    // Write the header
    csv_writer.write_record(&["Resource", "Action", "Role1", "Role2"])?;

    // Write rows
    for row in rows {
        csv_writer.write_record(&[
            &row.resource,
            &row.action,
            &row.has_capability1.to_string(),
            &row.has_capability2.to_string(),
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}

#[test]
fn test_write_csv_empty_rows() {
    let rows: Vec<CapabilityRow> = Vec::new();
    let mut writer = Cursor::new(Vec::new());

    assert!(write_csv(&rows, &mut writer).is_ok());

    let content = String::from_utf8(writer.into_inner()).unwrap();
    assert_eq!(content, "Resource,Action,Role1,Role2\n");
}

// Test case: single row
#[test]
fn test_write_csv_single_row() {
    let rows = vec![CapabilityRow {
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
        CapabilityRow {
            resource: String::from("Resource1"),
            action: String::from("Action1"),
            has_capability1: true,
            has_capability2: false,
        },
        CapabilityRow {
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
