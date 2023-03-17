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
