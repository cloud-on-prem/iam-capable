use capability::{CapabilityRow, extract_capabilities_from_policies};
use structopt::clap::ArgMatches;
use structopt::StructOpt;

mod aws;
mod compare;
mod output;
mod policy;
mod cli;
mod capability;


use aws::client::get_aws_client;
use output::format::OutputFormat;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long)]
    role1: String,
    #[structopt(long)]
    role2: String,
    #[structopt(long, default_value = "csv", parse(try_from_str))]
    output_format: OutputFormat,
}

#[tokio::main]
async fn main() {
    let arg_matches: ArgMatches = cli::fetch_cli_args().into();
    let aws_client = get_aws_client().await.unwrap();

    match arg_matches.subcommand() {
        ("compare", Some(compare_matches)) => {
            let role1 = compare_matches.value_of("role1").unwrap();
            let role2 = compare_matches.value_of("role2").unwrap();
            let output_format: OutputFormat = compare_matches
                .value_of("output-format")
                .unwrap()
                .parse()
                .unwrap();
            let policy1 = policy::fetch_role_policy(&aws_client.iam, &aws_client.account_id, role1)
                .await
                .unwrap();
            let policy2 = policy::fetch_role_policy(&aws_client.iam, &aws_client.account_id, role2)
                .await
                .unwrap();
            let rows: Vec<CapabilityRow> = compare::compare_policies(policy1, policy2);

            output::format::print(output_format, &rows)
        },
        ("fetch", Some(fetch_matches)) => {
            let role = fetch_matches.value_of("role").unwrap();
            let output_format: OutputFormat = fetch_matches
                .value_of("output-format")
                .unwrap()
                .parse()
                .unwrap();

            // Fetch the policies for the single role
            let policies = policy::fetch_role_policy(&aws_client.iam, &aws_client.account_id, role).await.unwrap();

            let rows = extract_capabilities_from_policies(policies);
            output::format::print(output_format, &rows);
        }
        _ => {
            eprintln!("Unknown subcommand. Use --help for more information.");
            std::process::exit(1);
        }
    }
}
