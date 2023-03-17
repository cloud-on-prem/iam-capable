# IAM Capable 💪

A command-line tool for comparing the permissions and capabilities of two IAM Roles. (Currently, only AWS IAM is supported)

## Description

This tool retrieves the policies attached to two AWS IAM roles and displays a comparison table of their capabilities. The tool helps ensure that two roles have the same or similar permissions or to understand the differences between them.

## Features

- Fetches policies associated with IAM roles
- Compares policies to generate a table of differences
- Outputs comparison results in CSV or JSON format

## Dependencies

- [Rust](https://www.rust-lang.org/)

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone this repository: `git clone https://github.com/yourusername/iam-capable.git`
3. Change to the repository directory: `cd iam-capable.`
4. Build the project: `cargo build --release`
5. The binary will be available at `./target/release/iam-capable`

## Usage

0. Ensure the shell is configured to use AWS Credentials. Follow the [instructions here](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-quickstart.html) to set it up.
1. Run the following command

  ```sh
    iam-capable --role1 ROLE1_NAME --role2 ROLE2_NAME [--output_format OUTPUT_FORMAT]
    # - ROLE1_NAME: The name of the first IAM role to compare.
    # - ROLE2_NAME: The name of the second IAM role to compare.
    # - OUTPUT_FORMAT (optional): The output format for the comparison results. Available formats: CSV (default), JSON.
  ```

## Example

```sh
  iam-capable --role1 myrole1 --role2 myrole2 --output_format json
```

## Sample Output (when piped to [csvkit](https://csvkit.readthedocs.io/en/latest/))

```sh
$ iam-capable --role1 myrole1 --role2 myrole2 --output_format csv | csvlook

| Resource                   | Action              | Role1 | Role2 |
|----------------------------|---------------------|-------|-------|
| arn:aws:s3:::my-bucket/*   | s3:ListBucket       | true  | false |
| arn:aws:s3:::my-bucket/*   | s3:GetObject        | true  | true  |
| arn:aws:s3:::my-bucket/*   | s3:PutObject        | false | true  |
| arn:aws:ec2:*:*:instance/* | ec2:StartInstances  | true  | false |
| arn:aws:ec2:*:*:instance/* | ec2:StopInstances   | true  | true  |
| arn:aws:ec2:*:*:instance/* | ec2:RebootInstances | false | true  |
```

---

## License

MIT
