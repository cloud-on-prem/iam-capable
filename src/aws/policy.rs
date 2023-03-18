use crate::aws::iam::Policy;
use aws_sdk_iam::Client as IamClient;
use serde_json::Value;
use url::form_urlencoded;

/// Fetches the policy statements for a given IAM Role.
///
/// # Arguments
///
/// * `iam` - A reference to an IamClient instance.
/// * `account_id` - A string slice containing the AWS account ID.
/// * `role_name` - A string slice containing the IAM Role name.
///
/// # Returns
///
/// A Result containing a Vec of Policy, or an Error.
pub async fn fetch_role_policy(
    iam: &IamClient,
    account_id: &str,
    role_name: &str,
) -> Result<Vec<Policy>, Box<dyn std::error::Error + Send + Sync>> {
    let attached_policy_arns = fetch_attached_policy_arns(iam, role_name).await?;
    let inline_policy_arns = fetch_inline_policy_arns(iam, account_id, role_name).await?;

    let policy_arns = attached_policy_arns
        .into_iter()
        .chain(inline_policy_arns.into_iter());

    let mut policies = Vec::<Policy>::new();

    for policy_arn in policy_arns {
        let json_document = fetch_json_policy_document(iam, &policy_arn).await?;
        policies.push(json_document);
    }

    Ok(policies)
}

async fn fetch_attached_policy_arns(
    iam: &IamClient,
    role_name: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let attached_policies = iam
        .list_attached_role_policies()
        .role_name(role_name)
        .send()
        .await?;

    let policy_arns: Vec<String> = attached_policies
        .attached_policies
        .unwrap_or_default()
        .into_iter()
        .filter_map(|policy| policy.policy_arn)
        .collect();

    Ok(policy_arns)
}

async fn fetch_inline_policy_arns(
    iam: &IamClient,
    account_id: &str,
    role_name: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let inline_policies = iam.list_role_policies().role_name(role_name).send().await?;

    let inline_policy_arns: Vec<String> = inline_policies
        .policy_names
        .unwrap_or_default()
        .into_iter()
        .map(|policy_name| {
            format!(
                "arn:aws:iam::{}:role/{}/{}",
                account_id, role_name, policy_name
            )
        })
        .collect();

    Ok(inline_policy_arns)
}

async fn fetch_json_policy_document(
    iam: &IamClient,
    policy_arn: &str,
) -> Result<Policy, Box<dyn std::error::Error + Send + Sync>> {
    let document = fetch_policy_document(iam, policy_arn).await?;

    let decoded_document: String = form_urlencoded::parse(document.as_bytes())
        .map(|(key, _)| key)
        .collect();

    let json_document: Value = serde_json::from_str(&decoded_document)?;
    let policy: Policy = serde_json::from_value(json_document)?;

    Ok(policy)
}

/// Fetches the policy document for a given policy ARN, focusing on the default policy version.
///
/// # Arguments
///
/// * `iam` - A reference to an IamClient instance.
/// * `policy_arn` - A string slice containing the policy ARN.
///
/// # Returns
///
/// A Result containing a String representing the policy document, or an Error.
async fn fetch_policy_document(
    iam: &IamClient,
    policy_arn: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let policy_versions_output = iam
        .list_policy_versions()
        .policy_arn(policy_arn)
        .send()
        .await?;

    let policy_versions = policy_versions_output.versions.unwrap_or_default();

    let default_policy_version = policy_versions
        .iter()
        .find(|version| version.is_default_version);

    match default_policy_version {
        Some(version) => {
            let policy_version_id = version.version_id.as_ref().ok_or_else(|| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to get policy version ID",
                ))
            })?;
            let policy_version_output = iam
                .get_policy_version()
                .policy_arn(policy_arn)
                .version_id(policy_version_id)
                .send()
                .await?;

            let document = policy_version_output
                .policy_version
                .and_then(|pv| pv.document)
                .ok_or("Failed to get policy document")?;

            Ok(document)
        }
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to find default policy version",
        ))),
    }
}
