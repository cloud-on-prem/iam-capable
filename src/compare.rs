// use colored::*;
use cli_table::{format::Justify, Cell, Table};
use cli_table::{print_stdout, Style};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Capability {
    resource: String,
    action: String,
}

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

    display_comparison_table(&capabilities1, &capabilities2, role1, role2);
}

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

fn display_comparison_table(
    capabilities1: &HashMap<Capability, bool>,
    capabilities2: &HashMap<Capability, bool>,
    role1: &str,
    role2: &str,
) {
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

    let rows: Vec<Vec<_>> = all_keys
        .into_iter()
        .filter_map(|key| {
            let has_capability1 = capabilities1.get(key).unwrap_or(&false);
            let has_capability2 = capabilities2.get(key).unwrap_or(&false);

            if !(*has_capability1 && *has_capability2) {
                Some(vec![
                    key.resource.clone().cell(),
                    key.action.clone().cell(),
                    has_capability1.cell().justify(Justify::Center),
                    has_capability2.cell().justify(Justify::Center),
                ])
            } else {
                None
            }
        })
        .collect();

    let table = rows
        .table()
        .title(vec![
            "Resource".cell().bold(true).justify(Justify::Center),
            "Action".cell().bold(true).justify(Justify::Center),
            role1.cell().bold(true).justify(Justify::Center),
            role2.cell().bold(true).justify(Justify::Center),
        ])
        .bold(true);

    assert!(print_stdout(table).is_ok());
}
