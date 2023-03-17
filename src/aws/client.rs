use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iam::Client as IamClient;
use aws_sdk_sts::Client as StsClient;
use std::error::Error;

pub struct AwsClient {
    pub iam: IamClient,
    pub account_id: String,
}

pub async fn get_aws_client() -> Result<AwsClient, Box<dyn Error + Send + Sync>> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let iam = IamClient::new(&config);
    let sts = StsClient::new(&config);
    let account_id = fetch_account_id(&sts).await;
    match account_id {
        Ok(account_id) => Ok(AwsClient { iam, account_id }),
        Err(e) => Err(e),
    }
}

async fn fetch_account_id(
    sts: &StsClient,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let account_id =
        sts.get_caller_identity().send().await?.account.ok_or(
            "Could not fetch account ID. Please check your AWS credentials and try again.",
        )?;
    Ok(account_id)
}
