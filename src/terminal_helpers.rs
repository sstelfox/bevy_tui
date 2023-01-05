use std::error::Error;
use std::io::Write;

use crossterm::{cursor, QueueableCommand};
use crossterm::event::{
    EnableBracketedPaste, EnableFocusChange, EnableMouseCapture,
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::CrosstermBackend;

use crate::{BevyTerminal, Terminal};

pub fn create_terminal() -> Result<BevyTerminal, Box<dyn Error>> {
    let stdout = std::io::stdout();

    let backend = CrosstermBackend::new(stdout);
    let terminal = tui::Terminal::new(backend)?;

    Ok(Terminal(terminal))
}

pub fn initialize_terminal() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = std::io::stdout();

    stdout.queue(EnterAlternateScreen)?;
    stdout.queue(EnableBracketedPaste)?;
    stdout.queue(EnableFocusChange)?;
    stdout.queue(EnableMouseCapture)?;

    //stdout.queue(crossterm::event::PushKeyboardEnhancementFlags(
    //    crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
    //        | crossterm::event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES
    //        | crossterm::event::KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
    //))?;

    stdout.flush().expect("terminal command trigger");

    Ok(())
}

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
