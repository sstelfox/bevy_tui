//! Basic TUI rendering example for Bevy

use bevy::prelude::*;

use bevy::app::AppExit;
use bevy_tui::MinimalTuiPlugins;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Perform the initial setup of the terminal such as enabling raw mode, switching to the
    // alternate mode, enabling mouse control and more.
    bevy_tui::initialize_terminal()?;

    // MinimalTuiPlugins is the equivalent of Bevy's MinimalPlugins but with a replaced scheduler
    // and input handling.
    App::new()
        .add_plugins(MinimalTuiPlugins)
        .add_system(run_basic_ui)
        .add_system(quit_on_esc)
        .run();

    // The changes to the terminal need to be undone before returning the terminal for interactive
    // use. Without doing this the terminal will be in a weird mostly unusable state until it has
    // been reset.
    bevy_tui::teardown_terminal()?;

    Ok(())
}

fn quit_on_esc(key_code: Res<Input<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_code.just_pressed(KeyCode::Q) {
        event_writer.send(AppExit);
    }
}

fn render_ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>) {
    let chunks = tui::layout::Layout::default()
        .constraints([tui::layout::Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let styled_content = tui::text::Span::styled("Hello Bevy! Press 'q' to quit.", tui::style::Style::default());
    let paragraph = tui::widgets::Paragraph::new(styled_content)
        .alignment(tui::layout::Alignment::Center)
        .wrap(tui::widgets::Wrap { trim: true });

    f.render_widget(paragraph, chunks[0]);
}

fn run_basic_ui(mut terminal: ResMut<bevy_tui::BevyTerminal>) {
    terminal
        .0
        .draw(|f| render_ui(f))
        .expect("failed to draw to terminal");
}
