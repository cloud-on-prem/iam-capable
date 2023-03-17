use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::aws::iam::Policy;
use crate::aws::iam::PolicyStatement;

/// Represents an AWS capability, consisting of a resource and an action.
#[derive(Hash, PartialEq, Eq, Debug)]
struct Capability {
    resource: String,
    action: String,
}

/// Represents a row in the comparison table.
pub struct CapabilityRow {
    pub resource: String,
    pub action: String,
    pub has_capability1: bool,
    pub has_capability2: bool,
}

/// Compares two sets of policies and outputs a table displaying their differences.
pub fn compare_policies(policies1: Vec<Policy>, policies2: Vec<Policy>) -> Vec<CapabilityRow> {
    let mut capabilities1 = HashMap::<Capability, bool>::new();
    let mut capabilities2 = HashMap::<Capability, bool>::new();

    for policy1 in policies1 {
        let statements1 = policy1.statement;
        let policy_capabilities1 = extract_capabilities(statements1);
        capabilities1.extend(policy_capabilities1);
    }

    for policy2 in policies2 {
        let statements2 = policy2.statement;
        let policy_capabilities2 = extract_capabilities(statements2);
        capabilities2.extend(policy_capabilities2);
    }

    let all_keys_set: HashSet<&Capability> =
        HashSet::from_iter(capabilities1.keys().chain(capabilities2.keys()));
    let mut all_keys: Vec<&Capability> = all_keys_set.into_iter().collect();
    all_keys.sort_by(|a, b| {
        let res = a.resource.cmp(&b.resource);
        if res == std::cmp::Ordering::Equal {
            a.action.cmp(&b.action)
        } else {
            res
        }
    });

    let capability_rows: Vec<CapabilityRow> = all_keys
        .into_iter()
        .filter_map(|key| {
            let has_capability1 = capabilities1.get(key).unwrap_or(&false);
            let has_capability2 = capabilities2.get(key).unwrap_or(&false);

            if !(*has_capability1 && *has_capability2) {
                Some(CapabilityRow {
                    resource: key.resource.clone(),
                    action: key.action.clone(),
                    has_capability1: has_capability1.clone(),
                    has_capability2: has_capability2.clone(),
                })
            } else {
                None
            }
        })
        .collect();

    capability_rows
}

/// Extracts the capabilities from the policy statements.
fn extract_capabilities(statements: Vec<PolicyStatement>) -> HashMap<Capability, bool> {
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
