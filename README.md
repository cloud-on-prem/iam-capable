# AWS IAM Role Diff

A command-line tool for comparing the permissions of two AWS IAM Roles.

## Description

This tool retrieves the policies attached to two AWS IAM roles and displays a comparison table of their capabilities. This is useful for ensuring that two roles have the same or similar permissions or for understanding the differences between them.

## Features

- Fetches policies associated with IAM roles
- Compares policies to generate a table of differences
- Outputs comparison results in CSV or JSON format

## Dependencies

- [Rust](https://www.rust-lang.org/)

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone this repository: `git clone https://github.com/yourusername/aws-iam-role-diff.git`
3. Change to the repository directory: `cd aws-iam-role-diff`
4. Build the project: `cargo build --release`
5. The binary will be available at `./target/release/aws-iam-role-diff`

## Usage

```sh
  ./aws-iam-role-diff --role1 ROLE1_NAME --role2 ROLE2_NAME [--output_format OUTPUT_FORMAT]
  # - ROLE1_NAME: The name of the first IAM role to compare.
  # - ROLE2_NAME: The name of the second IAM role to compare.
  # - OUTPUT_FORMAT (optional): The output format for the comparison results. Available formats: csv (default), json.
```

## Example

```sh
  ./aws-iam-role-diff --role1 myrole1 --role2 myrole2 --output_format json
```
