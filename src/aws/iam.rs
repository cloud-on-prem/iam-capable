use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde::Deserialize as SerdeDeserialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Policy {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Statement")]
    pub statements: Vec<PolicyStatement>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Effect {
    Allow,
    Deny
} 

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PolicyStatement {
    #[serde(rename = "Effect")]
    pub effect: Effect,
    #[serde(rename = "Action")]
    #[serde(deserialize_with = "deserialize_string_or_array")]
    pub action: Vec<String>,
    #[serde(rename = "Resource")]
    #[serde(deserialize_with = "deserialize_string_or_array")]
    pub resource: Vec<String>,
}

fn deserialize_string_or_array<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(vec![s]),
        Value::Array(arr) => Ok(arr
            .into_iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect()),
        _ => Err(serde::de::Error::custom("expected string or array of strings")),
    }
}
