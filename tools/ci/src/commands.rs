use std::process::Command;

use crate::constants::CiCheck;
use crate::error::CommandError;

struct CheckCommand {
    check_type: CiCheck,
    base_command: &'static str,
    arguments: &'static [&'static str],
}

impl CheckCommand {
    const fn new(
        check_type: CiCheck,
        base_command: &'static str,
        arguments: &'static [&'static str],
    ) -> Self {
        Self {
            check_type,
            base_command,
            arguments,
        }
    }
}

impl CheckCommand {
    fn full_command(&self) -> String {
        format!("{} {}", self.base_command, self.arguments.join(" "))
    }
}

const CI_COMMANDS: &[CheckCommand] = &[
    // Confirm the repository matches our configured lint rules, lint configuration takes place in
    // `/.cargo/config.toml`
    CheckCommand::new(
        CiCheck::CODE_LINTING,
        "cargo",
        &["clippy", "--workspace", "--all-targets", "--all-features"],
    ),
    // Run a security audit on the dependencies
    CheckCommand::new(CiCheck::DEPENDENCY_AUDIT, "cargo", &["audit"]),
    // Cargo deny provides additional dependency audit checks on the repository that audit alone
    // doens't provide
    CheckCommand::new(CiCheck::DEPENDENCY_AUDIT, "cargo", &["deny", "check"]),
    // Validate that the examples compile
    // TODO: Need to switch to 'benches' directory
    CheckCommand::new(
        CiCheck::CHECK_BENCHMARKS,
        "cargo",
        &["check", "--workspace", "--benches"],
    ),
    // Check that all the docs can be generated without warnings
    // TODO: should set environment RUSTDOCFLAGS="-Dwarnings"
    CheckCommand::new(
        CiCheck::CHECK_DOCUMENTATION,
        "cargo",
        &[
            "doc",
            "--workspace",
            "--all-features",
            "--no-deps",
            "--document-private-items",
            "--examples",
        ],
    ),
    // Validate that the project compiles normally
    CheckCommand::new(CiCheck::CHECK_EXAMPLES, "cargo", &["check", "--workspace"]),
    // Validate that the examples compile
    CheckCommand::new(
        CiCheck::CHECK_EXAMPLES,
        "cargo",
        &["check", "--workspace", "--examples"],
    ),
    // Confirm all the code is formatted correctly in the repository
    CheckCommand::new(
        CiCheck::CHECK_FORMATTING,
        "cargo",
        &["fmt", "--all", "--", "--check"],
    ),
    // Run any tests present in the documentation
    CheckCommand::new(
        CiCheck::RUN_DOCUMENTATION_TESTS,
        "cargo",
        &["test", "--workspace", "--doc"],
    ),
    // Run all the standard unit tests, does not build examples or run doc tests
    CheckCommand::new(
        CiCheck::RUN_UNIT_TESTS,
        "cargo",
        &[
            "test",
            "--workspace",
            "--lib",
            "--bins",
            "--tests",
            "--benches",
        ],
    ),
    // Run all the tests through the miri inspector as well
    // TODO: might be worth controlling the seed to ensure runs vary
    // TODO: Might need to set a different CARGO_TARGET_DIR for this or do a `cargo clean` before
    // it runs... Shouldn't matter for now as the job runs fresh in our CI flow
    CheckCommand::new(
        CiCheck::RUN_UNDEFINED_BEHAVIOR_TESTS,
        "cargo",
        &[
            "miri",
            "test",
            "--workspace",
            "--lib",
            "--bins",
            "--tests",
            "--benches",
        ],
    ),
];

pub(crate) struct CommandRunDetails {
    error: CommandError,

    stdout: Option<String>,
    stderr: Option<String>,

    status_code: Option<i32>,
}

impl CommandRunDetails {
    pub(crate) fn error(&self) -> &CommandError {
        &self.error
    }

    fn set_status_code(&mut self, status_code: i32) {
        self.status_code = Some(status_code);
    }

    fn set_stderr(&mut self, stderr: String) {
        self.stderr = Some(stderr);
    }

    fn set_stdout(&mut self, stdout: String) {
        self.stdout = Some(stdout);
    }

    pub(crate) fn status_code(&self) -> Option<i32> {
        self.status_code
    }

    pub(crate) fn stderr(&self) -> Option<&String> {
        self.stderr.as_ref()
    }

    pub(crate) fn stdout(&self) -> Option<&String> {
        self.stdout.as_ref()
    }
}

impl From<CommandError> for CommandRunDetails {
    fn from(value: CommandError) -> Self {
        Self {
            error: value,

            stdout: None,
            stderr: None,

            status_code: None,
        }
    }
}

pub(crate) struct Runner {
    checks: CiCheck,
}

impl Runner {
    pub(crate) fn execute(&self) -> Result<(), Vec<CommandRunDetails>> {
        let mut failures = vec![];

        for check_command in CI_COMMANDS.iter() {
            if self.checks.contains(check_command.check_type) {
                println!(
                    "Running {:?}({})...",
                    check_command.check_type,
                    check_command.full_command()
                );

                let result = Command::new(check_command.base_command)
                    .args(check_command.arguments)
                    .output();

                let output = match result {
                    Ok(out) => out,
                    Err(err) => {
                        println!(
                            "Failed to execute CI command '{}': {}",
                            check_command.full_command(),
                            err
                        );
                        continue;
                    }
                };

                if output.status.success() {
                    continue;
                }

                let mut run_details =
                    CommandRunDetails::from(check_command.check_type.error_type());

                if let Some(code) = output.status.code() {
                    run_details.set_status_code(code);
                }

                match std::str::from_utf8(&output.stdout) {
                    Ok(s) => {
                        if !s.is_empty() {
                            run_details.set_stdout(s.to_string());
                        }
                    }
                    Err(_) => {
                        println!(
                            "ERROR! STDOUT of CI check '{:?}' produced non-UTF8 output",
                            check_command.check_type
                        );
                    }
                }

                match std::str::from_utf8(&output.stderr) {
                    Ok(s) => {
                        if !s.is_empty() {
                            run_details.set_stderr(s.to_string());
                        }
                    }
                    Err(_) => {
                        println!(
                            "ERROR! STDERR of CI check '{:?}' produced non-UTF8 output",
                            check_command.check_type
                        );
                    }
                }

                failures.push(run_details);
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures)
        }
    }

    pub(crate) fn new(checks: CiCheck) -> Self {
        Self { checks }
    }
}
