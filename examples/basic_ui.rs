//! Basic TUI rendering example for Bevy

use bevy::prelude::*;
use bevy::core::CorePlugin;
use bevy::time::TimePlugin;
use bevy_tui::TuiPlugin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(TuiPlugin::default())
        .add_system(run_basic_ui)
        .run();

    // The TuiPlugin handles the initialization of the terminal itself, as there aren't "shutdown"
    // systems in Bevy this needs to be handled manually.
    bevy_tui::teardown_terminal()?;

    Ok(())
}

fn run_basic_ui(mut terminal: ResMut<bevy_tui::BevyTerminal>) {
    terminal.0.draw(|f| render_ui(f)).expect("failed to draw to terminal");
}

fn render_ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>) {
    let chunks = tui::layout::Layout::default()
        .constraints([tui::layout::Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let styled_content = tui::text::Span::styled("Hello Bevy!", tui::style::Style::default());
    let paragraph = tui::widgets::Paragraph::new(styled_content)
        .alignment(tui::layout::Alignment::Center)
        .wrap(tui::widgets::Wrap { trim: true });

    f.render_widget(paragraph, chunks[0]);
}
