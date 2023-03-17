use crate::compare::CapabilityRow;
use csv::Writer;
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
            &row.has_capability1,
            &row.has_capability2,
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}
