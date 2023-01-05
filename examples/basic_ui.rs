//! Basic TUI rendering example for Bevy

use bevy::prelude::*;

use bevy::app::AppExit;
use bevy_tui::prelude::*;

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Layout};
use tui::style::Style;
use tui::text::Span;
use tui::widgets::{Paragraph, Wrap};
use tui::Frame;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Perform the initial setup of the terminal such as enabling raw mode, switching to the
    // alternate mode, enabling mouse control and more.
    initialize_terminal()?;

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
    teardown_terminal()?;

    Ok(())
}

fn quit_on_esc(key_code: Res<Input<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_code.just_pressed(KeyCode::Q) {
        event_writer.send(AppExit);
    }
}

fn render_ui<B: Backend>(f: &mut Frame<B>, input: &Input<KeyCode>) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(1), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let hello_content = Span::styled("Hello Bevy! Press 'q' to quit.", Style::default());
    let hello_paragraph = Paragraph::new(hello_content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(hello_paragraph, chunks[0]);

    let input_content = Span::styled(format!("{:?}", input), Style::default());
    let input_paragraph = Paragraph::new(input_content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(input_paragraph, chunks[1]);
}

fn run_basic_ui(mut terminal: ResMut<bevy_tui::BevyTerminal>, current_input: Res<Input<KeyCode>>) {
    terminal
        .0
        .draw(|f| render_ui(f, &current_input))
        .expect("failed to draw to terminal");
}
