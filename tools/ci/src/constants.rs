use bitflags::bitflags;
use lazy_static::lazy_static;

use crate::error::CommandError;

bitflags! {
    pub(crate) struct CiCheck: u32 {
        const CODE_LINTING = 0b0000_0000_0001;
        const DEPENDENCY_AUDIT = 0b0000_0000_0010;

        const CHECK_BENCHMARKS = 0b0000_0000_0100;
        const CHECK_COMPILATION = 0b0000_0001_0000;
        const CHECK_DOCUMENTATION = 0b0000_0000_1000;
        const CHECK_EXAMPLES = 0b0000_0010_0000;
        const CHECK_FORMATTING = 0b0000_0100_0000;

        const RUN_DOCUMENTATION_TESTS = 0b0000_1000_0000;
        const RUN_UNIT_TESTS = 0b0001_0000_0000;
        const RUN_UNDEFINED_BEHAVIOR_TESTS = 0b0010_0000_0000;
    }
}

impl CiCheck {
    pub(crate) fn error_type(self) -> CommandError {
        match self {
            CiCheck::CODE_LINTING => CommandError::LintFailure,
            CiCheck::DEPENDENCY_AUDIT => CommandError::SecurityAuditFailure,

            CiCheck::CHECK_BENCHMARKS => CommandError::InvalidBenchmarks,
            CiCheck::CHECK_COMPILATION => CommandError::CompilationFailed,
            CiCheck::CHECK_DOCUMENTATION => CommandError::InvalidDocumentation,
            CiCheck::CHECK_EXAMPLES => CommandError::InvalidExamples,
            CiCheck::CHECK_FORMATTING => CommandError::UnformattedCode,

            CiCheck::RUN_DOCUMENTATION_TESTS => CommandError::FailedDocumentationTests,
            CiCheck::RUN_UNIT_TESTS => CommandError::FailedUnitTests,
            CiCheck::RUN_UNDEFINED_BEHAVIOR_TESTS => CommandError::DetectedUndefinedBehavior,

            // TODO: flesh out the error types for the rest of the checks
            _ => CommandError::UnmatchedCommandCheck(self.bits()),
        }
    }
}

lazy_static! {
    pub(crate) static ref ARGUMENT_CHECK_MAP: [(&'static str, CiCheck); 11] = [
        // When unspecified we run all the checks, but we normally want the miri checks split off
        // into their own so the compilation cache isn't used (this will be the same with code
        // coverage when that is included). So our default set is everything except the special
        // casees
        ("default", CiCheck::all() - CiCheck::RUN_UNDEFINED_BEHAVIOR_TESTS),

        ("compile", CiCheck::CHECK_BENCHMARKS | CiCheck::CHECK_EXAMPLES | CiCheck::CHECK_COMPILATION),
        ("examples", CiCheck::CHECK_EXAMPLES),
        ("lints", CiCheck::CHECK_FORMATTING | CiCheck::CODE_LINTING | CiCheck::DEPENDENCY_AUDIT),

        // I want all of these CLI arguments to largely match the default rust equivalents
        ("audit", CiCheck::DEPENDENCY_AUDIT),
        ("bench", CiCheck::CHECK_BENCHMARKS),
        ("clippy", CiCheck::CODE_LINTING),
        ("doc", CiCheck::CHECK_DOCUMENTATION | CiCheck::RUN_DOCUMENTATION_TESTS),
        ("fmt", CiCheck::CHECK_FORMATTING),
        ("miri", CiCheck::RUN_UNDEFINED_BEHAVIOR_TESTS),
        ("test", CiCheck::RUN_UNIT_TESTS),
    ];
}
