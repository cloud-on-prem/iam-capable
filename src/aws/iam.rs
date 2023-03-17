use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    pub version: String,
    pub statement: Vec<PolicyStatement>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyStatement {
    pub action: Vec<String>,
    pub effect: String,
    pub resource: Vec<String>,
}
