use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

/// This is the core error type inside the crate.
#[derive(Debug)]
pub enum Error {
    /// This is an extreme and odd error to encounter. If this occurs the Bevy system did not have
    /// an event queue for application exit events. This should never happen without a really
    /// messed up system.
    MissingExitEventQueue,

    /// While attempting to retrieve terminal events an error occurred.
    PollFailed(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::MissingExitEventQueue => write!(f, "missing AppExit event queue in world"),
            Self::PollFailed(err) => write!(f, "failed to poll terminal: {err}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::MissingExitEventQueue => None,
            Self::PollFailed(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::PollFailed(err)
    }
}
