use structopt::clap::{App, Arg, SubCommand, ArgMatches};

pub fn fetch_cli_args() ->  ArgMatches<'static> {
    App::new("IAM Capable")
        .version("1.0")
        .about("A tool to compare and fetch IAM Role capabilities")
        .subcommand(
            SubCommand::with_name("compare")
                .about("Compares two IAM Roles")
                .arg(
                    Arg::with_name("role1")
                        .long("role1")
                        .value_name("ROLE1")
                        .help("Name of the first IAM Role")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("role2")
                        .long("role2")
                        .value_name("ROLE2")
                        .help("Name of the second IAM Role")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output-format")
                        .long("output-format")
                        .short("o")
                        .value_name("FORMAT")
                        .help("Output format, either 'json' or 'csv'")
                        .takes_value(true)
                        .default_value("csv"),
                ),
        )
        .subcommand(
            SubCommand::with_name("fetch")
                .about("Fetches capabilities for a single IAM Role")
                .arg(
                    Arg::with_name("role")
                        .long("role")
                        .value_name("ROLE")
                        .help("Name of the IAM Role")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output-format")
                        .long("output-format")
                        .value_name("FORMAT")
                        .help("Output format, either 'json' or 'csv'")
                        .takes_value(true)
                        .default_value("csv"),
                ),
        )
        .get_matches()
}
