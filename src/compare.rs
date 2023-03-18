use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::aws::iam::Policy;
use crate::capability::extract_capabilities_from_policies;
use crate::capability::{CapabilityComparisonRow, CapabilityRow};

/// Represents an AWS capability, consisting of a resource and an action.

/// Compares two sets of policies and outputs a table displaying their differences.
pub fn compare_policies(
    policies1: Vec<Policy>,
    policies2: Vec<Policy>,
) -> Vec<CapabilityComparisonRow> {
    let mut capabilities1 = HashMap::<CapabilityRow, bool>::new();
    let mut capabilities2 = HashMap::<CapabilityRow, bool>::new();

    let caps_from_policies1 = extract_capabilities_from_policies(policies1);
    for cap in caps_from_policies1 {
        capabilities1.insert(cap, true);
    }

    let caps_from_policies2 = extract_capabilities_from_policies(policies2);
    for cap in caps_from_policies2 {
        capabilities2.insert(cap, true);
    }

    let all_keys_set: HashSet<&CapabilityRow> =
        HashSet::from_iter(capabilities1.keys().chain(capabilities2.keys()));
    let mut all_keys: Vec<&CapabilityRow> = all_keys_set.into_iter().collect();
    all_keys.sort_by(|a, b| {
        let res = a.resource.cmp(&b.resource);
        if res == std::cmp::Ordering::Equal {
            a.action.cmp(&b.action)
        } else {
            res
        }
    });

    let capability_rows: Vec<CapabilityComparisonRow> = all_keys
        .into_iter()
        .filter_map(|key| {
            let has_capability1 = capabilities1.get(key).unwrap_or(&false);
            let has_capability2 = capabilities2.get(key).unwrap_or(&false);

            if !(*has_capability1 && *has_capability2) {
                Some(CapabilityComparisonRow {
                    resource: key.resource.clone(),
                    action: key.action.clone(),
                    has_capability1: *has_capability1,
                    has_capability2: *has_capability2,
                })
            } else {
                None
            }
        })
        .collect();

    capability_rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aws::iam::{Effect, PolicyStatement};

    fn make_policy_statement(actions: Vec<&str>, resources: Vec<&str>) -> PolicyStatement {
        PolicyStatement {
            action: actions.into_iter().map(String::from).collect(),
            resource: resources.into_iter().map(String::from).collect(),
            effect: Effect::Allow,
        }
    }

    fn make_policy(statements: Vec<PolicyStatement>) -> Policy {
        Policy {
            statements,
            version: "2012-10-17".to_string(),
        }
    }

    #[test]
    fn test_compare_policies() {
        let policies1 = vec![make_policy(vec![make_policy_statement(
            vec!["s3:ListBucket"],
            vec!["arn:aws:s3:::mybucket"],
        )])];

        let policies2 = vec![make_policy(vec![make_policy_statement(
            vec!["s3:GetObject"],
            vec!["arn:aws:s3:::mybucket/*"],
        )])];

        let capability_rows = compare_policies(policies1, policies2);

        assert_eq!(capability_rows.len(), 2);

        assert_eq!(capability_rows[0].resource, "arn:aws:s3:::mybucket");
        assert_eq!(capability_rows[0].action, "s3:ListBucket");
        assert_eq!(capability_rows[0].has_capability1, true);
        assert_eq!(capability_rows[0].has_capability2, false);

        assert_eq!(capability_rows[1].resource, "arn:aws:s3:::mybucket/*");
        assert_eq!(capability_rows[1].action, "s3:GetObject");
        assert_eq!(capability_rows[1].has_capability1, false);
        assert_eq!(capability_rows[1].has_capability2, true);
    }
}
