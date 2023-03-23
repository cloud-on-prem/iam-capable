# IAM Capable 💪

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/cloud-on-prem/iam-capable/test.yml?style=for-the-badge) &nbsp;  ![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/cloud-on-prem/iam-capable?include_prereleases&style=for-the-badge) &nbsp; ![GitHub](https://img.shields.io/github/license/cloud-on-prem/iam-capable?style=for-the-badge)

A command-line tool for fetching and comparing the permissions and capabilities of IAM Roles. (Currently, only AWS IAM is supported)

## Description

This tool retrieves the policies attached to one or multiple AWS IAM roles and displays their capabilities. It supports fetching capabilities for a single role or comparing capabilities between two roles. The tool helps to ensure that roles have the correct permissions and to understand the differences between them.

## Features

- Fetches policies associated with IAM roles
- Compares policies to generate a table of differences (Returns a "symmetric difference" between the two roles. `role1 Δ role2` i.e, is the set of capabilities that are in either `role1` or `role2`, but not in both)
- Outputs comparison or fetched results in CSV or JSON format

## Usage

0. Download the binary for your OS from the [releases](https://github.com/cloud-on-prem/iam-capable/releases) page.

1. Ensure the shell is configured to use AWS Credentials. Follow the [instructions here](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-quickstart.html) to set it up.

2. Fetch the capabilities of a single IAM role:  
  
    ```sh
      iam-capable fetch --role ROLE_NAME [--output-format OUTPUT_FORMAT]
      # - ROLE_NAME: The name of the IAM role to fetch capabilities for.
      # - OUTPUT_FORMAT (optional): The output format for the fetched results. Available formats: CSV (default), JSON.
    ```  

3. Compare the capabilities of two IAM roles:

    ```sh
      iam-capable compare --role1 ROLE_NAME1 --role2 ROLE_NAME2 [--output-format OUTPUT_FORMAT]
      # - ROLE_NAME1: The name of the first IAM role to compare capabilities for.
      # - ROLE_NAME2: The name of the second IAM role to compare capabilities for.
      # - OUTPUT_FORMAT (optional): The output format for the fetched results. Available formats: CSV (default), JSON.
    ```

## Limitations

Currently, the tool does a "basic" comparison of policy statements. For example, it does not look at policy boundaries or conditions on statements.

## Sample Outputs (when piped to [csvkit](https://csvkit.readthedocs.io/en/latest/) or [jq](https://github.com/stedolan/jq))

```sh
$ iam-capable compare --role1 myrole1 --role2 myrole2 --output_format csv | csvlook

| Resource                   | Action              | Role1 | Role2 |
|----------------------------|---------------------|-------|-------|
| arn:aws:s3:::my-bucket/*   | s3:ListBucket       | true  | false |
| arn:aws:s3:::my-bucket/*   | s3:GetObject        | true  | true  |
| arn:aws:s3:::my-bucket/*   | s3:PutObject        | false | true  |
| arn:aws:ec2:*:*:instance/* | ec2:StartInstances  | true  | false |
| arn:aws:ec2:*:*:instance/* | ec2:StopInstances   | true  | true  |
| arn:aws:ec2:*:*:instance/* | ec2:RebootInstances | false | true  |

$ iam-capable fetch --role myrole1 --output_format json | jq .

[
  {
    "action": "s3:*",
    "resource": "*"
  }
]
```

## Development (or for building from source)

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone this repository: `git clone https://github.com/yourusername/iam-capable.git`
3. Change to the repository directory: `cd iam-capable.`
4. Build the project: `cargo build --release`
5. The binary will be available at `./target/release/iam-capable`

---

## License

MIT
