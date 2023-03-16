use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iam::Client as IamClient;
use aws_sdk_sts::Client as StsClient;
use structopt::StructOpt;

mod compare;
mod policy;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    role1: String,
    #[structopt(long)]
    role2: String,
}

async fn fetch_account_id(
    sts: &StsClient,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let account_id = sts.get_caller_identity().send().await?.account.unwrap();
    Ok(account_id)
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let iam = IamClient::new(&config);
    let sts = StsClient::new(&config);
    let account_id = fetch_account_id(&sts).await.unwrap();

    let policy1 = policy::fetch_role_policy(&iam, &account_id, &args.role1)
        .await
        .unwrap();
    let policy2 = policy::fetch_role_policy(&iam, &account_id, &args.role2)
        .await
        .unwrap();

    compare::compare_and_display_json(policy1, policy2)
}
