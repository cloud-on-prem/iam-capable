use serde_json::json;

use crate::{output::format::OutputSerializable, aws::iam::Policy};
use crate::aws::iam::PolicyStatement;


#[derive(Hash, PartialEq, Eq, Debug)]
pub struct CapabilityRow {
    pub resource: String,
    pub action: String,
}

impl OutputSerializable for CapabilityRow {
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
pub struct CapabilityComparisonRow {
    pub resource: String,
    pub action: String,
    pub has_capability1: bool,
    pub has_capability2: bool,
}

impl OutputSerializable for CapabilityComparisonRow {
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


pub fn extract_capabilities_from_policies(policies: Vec<Policy>) -> Vec<CapabilityRow> {
    let mut capabilities: Vec<CapabilityRow> = Vec::new();

    for policy in policies {
        for statement in policy.statements {
            let capabilities_from_statement = extract_capabilities_from_statement(vec![statement]);
            capabilities.extend(capabilities_from_statement);
        }
    }
    capabilities.sort_by(|a, b| match a.resource.cmp(&b.resource) {
        std::cmp::Ordering::Equal => a.action.cmp(&b.action),
        order => order,
    });

    capabilities
}

/// Extracts the capabilities from the policy statements.
fn extract_capabilities_from_statement(statements: Vec<PolicyStatement>) -> Vec<CapabilityRow> {
    let mut capabilities: Vec<CapabilityRow> = Vec::new();

    for stmt in statements {
        let actions = stmt.action;

        let resources = stmt.resource;
        for action in &actions {
            for resource in &resources {
                let capability = CapabilityRow {
                    resource: resource.clone(),
                    action: action.clone(),
                };
                capabilities.push(capability);
            }
        }
    }
    capabilities
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aws::iam::{Policy, PolicyStatement, Effect};

    #[test]
    fn test_extract_capabilities_from_policies() {
        let policies = vec![
            Policy {
                version: "2012-10-17".to_string(),
                statements: vec![
                    PolicyStatement {
                        effect: Effect::Allow,
                        action: vec!["s3:ListBucket".to_string(), "s3:GetObject".to_string()],
                        resource: vec!["arn:aws:s3:::my-bucket".to_string()],
                    },
                    PolicyStatement {
                        effect: Effect::Allow,
                        action: vec!["s3:GetObject".to_string()],
                        resource: vec!["arn:aws:s3:::my-bucket/*".to_string()],
                    },
                ],
            },
        ];

        let expected_capabilities = vec![
            CapabilityRow {
                resource: "arn:aws:s3:::my-bucket".to_string(),
                action: "s3:ListBucket".to_string(),
            },
            CapabilityRow {
                resource: "arn:aws:s3:::my-bucket".to_string(),
                action: "s3:GetObject".to_string(),
            },
            CapabilityRow {
                resource: "arn:aws:s3:::my-bucket/*".to_string(),
                action: "s3:GetObject".to_string(),
            },
        ];

        let capabilities = extract_capabilities_from_policies(policies);
        assert_eq!(capabilities, expected_capabilities);
    }
}
