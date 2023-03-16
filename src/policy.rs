use aws_sdk_iam::Client as IamClient;
use serde_json::Value;
use url::form_urlencoded;

pub async fn fetch_role_policy(
    iam: &IamClient,
    account_id: &str,
    role_name: &str,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
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
    // let policy_names: Vec<String> = policy_arns
    //     .into_iter()
    //     .chain(inline_policies.policy_names.unwrap_or_default().into_iter())
    //     .collect();
    let policy_arns = policy_arns
        .into_iter()
        .chain(inline_policy_arns.into_iter());

    let mut policies = Vec::<Value>::new();

    for policy_arn in policy_arns {
        let document = fetch_policy_document(&iam, &policy_arn).await?;

        let decoded_document: String = form_urlencoded::parse(document.as_bytes())
            .map(|(key, _)| key)
            .collect();

        let json_document: Value = serde_json::from_str(&decoded_document).unwrap();
        policies.push(json_document);
    }

    Ok(Value::Array(policies))
}

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

    // Filter policy versions to get the default one
    let default_policy_version = policy_versions
        .iter()
        .find(|version| version.is_default_version);

    match default_policy_version {
        Some(version) => {
            let policy_version_id = version.version_id.as_ref().unwrap();
            let policy_version_output = iam
                .get_policy_version()
                .policy_arn(policy_arn)
                .version_id(policy_version_id)
                .send()
                .await?;

            let document = policy_version_output
                .policy_version
                .map(|pv| pv.document)
                .flatten()
                .ok_or_else(|| "Failed to get policy document")?;

            Ok(document)
        }
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to find default policy version",
        ))),
    }
}
