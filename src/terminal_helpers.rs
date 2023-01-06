use std::error::Error;
use std::io::Write;

use crossterm::event::{
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
    EnableFocusChange, EnableMouseCapture,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{cursor, QueueableCommand};
use tui::backend::CrosstermBackend;

use crate::{BevyTerminal, Terminal};

/// Helper method for creating a crossterm backed TUI terminal object. Currently only the crossterm
/// backend is supported but this will be expanded once all of the minimal functionality has been
/// implemented to my satisfaction.
pub(crate) fn create_terminal() -> Result<BevyTerminal, Box<dyn Error>> {
    let stdout = std::io::stdout();

    let backend = CrosstermBackend::new(stdout);
    let terminal = tui::Terminal::new(backend)?;

    Ok(Terminal(terminal))
}

/// Performs the various escape sequences to the terminal connected to STDOUT to be used for a TUI
/// application such as enabling raw mode and requesting the common set of features this library
/// intends to support at a minimum such as mouse and keyboard support.
///
/// This does not handle additional terminal extensions, setting the title, or window dimensions
/// which is currently the responsibility of the end-user. Eventually additional helpers and
/// configuration will be exposed for these purposes.
///
/// # Errors
///
/// This performs a series of escape sequences against STDOUT, if an I/O error occurs while writing
/// out or flushing these various sequences to the terminal an `Err` will be returned.
pub fn initialize_terminal() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = std::io::stdout();

    stdout.queue(EnterAlternateScreen)?;
    stdout.queue(EnableBracketedPaste)?;
    stdout.queue(EnableFocusChange)?;
    stdout.queue(EnableMouseCapture)?;

    // TODO: Make this a setting for the application
    //stdout.queue(crossterm::terminal::SetTitle("Hello Bevy"))?;

    //stdout.queue(crossterm::event::PushKeyboardEnhancementFlags(
    //    crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
    //        | crossterm::event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES
    //        | crossterm::event::KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
    //))?;

    stdout.flush().expect("terminal command trigger");

    Ok(())
}

/// Bring the terminal back into a usable mode. This needs to be called before the application
/// exits if [`initialize_terminal`] has been called.
///
/// # Errors
///
/// This performs a series of escape sequences against STDOUT, if an I/O error occurs while writing
/// out or flushing these various sequences to the terminal an `Err` will be returned.
pub fn teardown_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;

    let mut stdout = std::io::stdout();
    stdout.queue(LeaveAlternateScreen)?;
    stdout.queue(DisableBracketedPaste)?;
    stdout.queue(DisableFocusChange)?;
    stdout.queue(DisableMouseCapture)?;
    //stdout.queue(crossterm::event::PopKeyboardEnhancementFlags)?;
    stdout.queue(cursor::Show)?;
    stdout.flush()?;

    Ok(())
}
