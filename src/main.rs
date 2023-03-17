use compare::CapabilityRow;
use std::io::stdout;
use structopt::StructOpt;

mod aws;
mod compare;
mod output;
mod policy;

use aws::client::get_aws_client;
use output::csv::write_csv;
use output::format::OutputFormat;
use output::json::write_json;

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
    let args = Cli::from_args();
    let aws_client = get_aws_client().await.unwrap();

    let policy1 = policy::fetch_role_policy(&aws_client.iam, &aws_client.account_id, &args.role1)
        .await
        .unwrap();

    let policy2 = policy::fetch_role_policy(&aws_client.iam, &aws_client.account_id, &args.role2)
        .await
        .unwrap();

    let rows: Vec<CapabilityRow> = compare::compare_policies(&policy1, &policy2);

    match args.output_format {
        OutputFormat::Csv => {
            // Output as CSV
            if let Err(e) = write_csv(&rows, stdout()) {
                eprintln!("Error writing CSV: {}", e);
            }
        }
        OutputFormat::Json => {
            // Output as JSON
            if let Err(e) = write_json(&rows, stdout()) {
                eprintln!("Error writing JSON: {}", e);
            }
        }
    }
}
