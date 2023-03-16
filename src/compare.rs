use cli_table::{format::Justify, print_stdout, Table, WithTitle};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

/// Represents an AWS capability, consisting of a resource and an action.
#[derive(Hash, PartialEq, Eq, Debug)]
struct Capability {
    resource: String,
    action: String,
}

/// Represents a row in the comparison table.
#[derive(Table)]
struct CapabilityRow {
    #[table(title = "Resource", justify = "Justify::Left")]
    resource: String,
    #[table(title = "Action", justify = "Justify::Left")]
    action: String,
    #[table(title = "Role 1", justify = "Justify::Center")]
    has_capability1: String,
    #[table(title = "Role 2", justify = "Justify::Center")]
    has_capability2: String,
}

/// Compares two sets of policies and outputs a table displaying their differences.
pub fn compare_policies(policies1: &[Value], policies2: &[Value], role1: &str, role2: &str) {
    let mut capabilities1 = HashMap::<Capability, bool>::new();
    let mut capabilities2 = HashMap::<Capability, bool>::new();

    for policy1 in policies1 {
        let v1 = &vec![];
        let statements1 = policy1["Statement"].as_array().unwrap_or(v1);
        let policy_capabilities1 = extract_capabilities(statements1);
        capabilities1.extend(policy_capabilities1);
    }

    for policy2 in policies2 {
        let v2 = &vec![];
        let statements2 = policy2["Statement"].as_array().unwrap_or(v2);
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
                    has_capability1: has_capability1.to_string(),
                    has_capability2: has_capability2.to_string(),
                })
            } else {
                None
            }
        })
        .collect();

    display_table(capability_rows);
}

/// Extracts the capabilities from the policy statements.
fn extract_capabilities(statements: &[Value]) -> HashMap<Capability, bool> {
    let mut capabilities: HashMap<Capability, bool> = HashMap::new();

    for stmt in statements {
        let actions = stmt["Action"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect::<Vec<_>>();

        let resources = stmt["Resource"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect::<Vec<_>>();
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

/// Displays the comparison table for two sets of policies.
fn display_table(capability_rows: Vec<CapabilityRow>) {
    let table = capability_rows.with_title();
    print_stdout(table).unwrap();
}
