use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum CommandError {
    #[error("Compilation of some units failed")]
    CompilationFailed,

    #[error("Miri detected undefined behavior in the codebase")]
    DetectedUndefinedBehavior,

    #[error("A regression in the documentation tests was detected")]
    FailedDocumentationTests,

    #[error("A regression in the unit tests was detected")]
    FailedUnitTests,

    #[error("The benchmarks failed to compile")]
    InvalidBenchmarks,

    #[error("There was a problem with the documentation formatting")]
    InvalidDocumentation,

    #[error("There examples failed to compile")]
    InvalidExamples,

    #[error("One or more clippy lints found an issue")]
    LintFailure,

    #[error("One or more dependencies has a reported security advisory")]
    SecurityAuditFailure,

    #[error("The code needs to be run through `cargo fmt --all`")]
    UnformattedCode,

    #[error("Unable to identify the appropriate error type for provided CiCheck({0}), but an error occurred")]
    UnmatchedCommandCheck(u32),
}
