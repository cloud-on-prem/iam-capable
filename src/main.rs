use capability::{CapabilityRow, extract_capabilities_from_policies};
use aws::client::get_aws_client;
use std::io::stdout;

mod aws;
mod compare;
mod output;
mod policy;
mod cli;
mod capability;


#[tokio::main]
async fn main() {
    let args = cli::fetch_cli_args();
    let aws_client = get_aws_client().await.unwrap();
    let mut writer = stdout();

    match args {
        cli::IamCapable::Compare {
            role1,
            role2,
            output_format,
        } => {
            let policy1 = policy::fetch_role_policy(&aws_client.iam, &aws_client.account_id, &role1)
                .await
                .unwrap();
            let policy2 = policy::fetch_role_policy(&aws_client.iam, &aws_client.account_id, &role2)
                .await
                .unwrap();
            let rows: Vec<CapabilityRow> = compare::compare_policies(policy1, policy2);

            output::format::print(output_format, &rows, &mut writer)
        }
        cli::IamCapable::Fetch { role, output_format } => {
            // Fetch the policies for the single role
            let policies = policy::fetch_role_policy(&aws_client.iam, &aws_client.account_id, &role).await.unwrap();

            let rows = extract_capabilities_from_policies(policies);
            output::format::print(output_format, &rows, &mut writer);
        }
    }
}
