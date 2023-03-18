use serde_json::json;

use crate::{output::format::OutputSerializable, aws::iam::Policy};
use crate::aws::iam::PolicyStatement;
use std::collections::HashMap;


#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Capability {
    pub resource: String,
    pub action: String,
}

impl OutputSerializable for Capability {
    fn csv_header() -> Vec<&'static str> {
        vec!["Resource", "Action"]
    }

    fn csv_record(&self) -> Vec<String> {
        vec![self.resource.clone(), self.action.clone()]
    }

    fn to_json_value(&self) -> serde_json::Value {
        json!({
            "resource": self.resource,
            "action": self.action
        })
    }
}

/// Represents a row in the comparison table.
#[derive (Debug, PartialEq, Eq, Clone)]
pub struct CapabilityRow {
    pub resource: String,
    pub action: String,
    pub has_capability1: bool,
    pub has_capability2: bool,
}

impl OutputSerializable for CapabilityRow {
    fn csv_header() -> Vec<&'static str> {
        vec!["Resource", "Action", "Role1", "Role2"]
    }

    fn csv_record(&self) -> Vec<String> {
        vec![
            self.resource.clone(),
            self.action.clone(),
            self.has_capability1.to_string(),
            self.has_capability2.to_string(),
        ]
    }

    fn to_json_value(&self) -> serde_json::Value {
        json!({
            "resource": self.resource,
            "action": self.action,
            "role1": self.has_capability1,
            "role2": self.has_capability2
        })
    }
}

pub fn extract_capabilities_from_policies(policy: Vec<Policy>) -> Vec<Capability> {
    let mut capabilities: Vec<Capability> = Vec::new();

    for policy in policy {
        for statement in policy.statements {
            for action in statement.action {
                for resource in statement.resource.clone() {
                    capabilities.push(Capability {
                        resource,
                        action: action.clone(),
                    });
                }
            }
        }
    }
    capabilities.sort_by(|a, b| a.resource.cmp(&b.resource));
    capabilities
}

/// Extracts the capabilities from the policy statements.
pub fn extract_capabilities(statements: Vec<PolicyStatement>) -> HashMap<Capability, bool> {
    let mut capabilities: HashMap<Capability, bool> = HashMap::new();

    for stmt in statements {
        let actions = stmt.action;

        let resources = stmt.resource;
        for action in &actions {
            for resource in &resources {
                let capability = Capability {
                    resource: resource.clone(),
                    action: action.clone(),
                };
                capabilities.insert(capability, true);
            }
        }
    }
    capabilities
}
