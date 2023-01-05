//! Automated CI system tooling
//!
//! To keep running all of our CI tasks consistently across several build systems as well as
//! individual developer systems, this Rust tooling script was setup to safely wrap all of the
//! various tasks intended to sanity check the code before merging and release. Changes to this
//! should be done carefully and require lead maintainer review to ensure consistency with the
//! project's overall goals.

mod commands;
mod constants;
mod error;

use constants::{CiCheck, ARGUMENT_CHECK_MAP};

fn main() {
    let argument_enabled_checks =
        std::env::args()
            .skip(1)
            .try_fold(CiCheck::empty(), |acc, arg| {
                if let Some((_, check)) = ARGUMENT_CHECK_MAP.iter().find(|(str, _)| *str == arg) {
                    Ok(acc | *check)
                } else {
                    Err(format!("Invalid argument '{arg}'"))
                }
            });

    let mut enabled_checks = match argument_enabled_checks {
        Ok(c) => c,
        Err(msg) => {
            println!("{msg}");

            let valid_args: Vec<&str> = ARGUMENT_CHECK_MAP.iter().map(|(s, _)| *s).collect();
            println!("Valid arguments are: {}", valid_args.join(", "));

            std::process::exit(127);
        }
    };

    // Run all the checks if none were specified...
    if enabled_checks.is_empty() {
        enabled_checks = CiCheck::all();
    }

    let runner = commands::Runner::new(enabled_checks);

    match runner.execute() {
        Ok(_) => {
            println!("All checks passed successfully!");
            std::process::exit(0);
        }
        Err(run_details) => {
            for failed_run in &run_details {
                if let Some(status) = failed_run.status_code() {
                    println!("{} (exit code: {status}):", failed_run.error());
                } else {
                    println!("{}:", failed_run.error());
                }

                if let Some(s) = failed_run.stdout() {
                    println!("STDOUT:\n{s}\n");
                }

                if let Some(s) = failed_run.stderr() {
                    println!("STDERR:\n{s}\n");
                }
            }

            match run_details.len().try_into() {
                Ok(len) => {
                    std::process::exit(len);
                }
                // The maximum number of errors that could be returned can never be longer than the
                // number of entries in [`crate::commands::CI_COMMANDS`] which for very obviously
                // practical reasons can never exceed i32::MAX
                Err(_) => {
                    unreachable!()
                }
            }
        }
    }
}
