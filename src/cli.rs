use structopt::StructOpt;
use crate::output::format::OutputFormat;

#[derive(StructOpt)]
#[structopt(name = "IAM Capable", about = "A tool to compare and fetch IAM Role capabilities")]
pub enum IamCapable {
    #[structopt(name = "compare", about = "Compares two IAM Roles")]
    Compare {
        #[structopt(long, help = "Name of the first IAM Role", required = true)]
        role1: String,

        #[structopt(long, help = "Name of the second IAM Role", required = true)]
        role2: String,

        #[structopt(
            long,
            short,
            help = "Output format, either 'json' or 'csv'",
            default_value = "csv",
            parse(try_from_str)
        )]
        output_format: OutputFormat,
    },

    #[structopt(name = "fetch", about = "Fetches capabilities for a single IAM Role")]
    Fetch {
        #[structopt(long, help = "Name of the IAM Role", required = true)]
        role: String,

        #[structopt(
            long,
            help = "Output format, either 'json' or 'csv'",
            default_value = "csv",
            parse(try_from_str)
        )]
        output_format: OutputFormat,
    },
}

pub fn fetch_cli_args() -> IamCapable {
    IamCapable::from_args()
}
