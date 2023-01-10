//! A more complex example that allows painting some minimal colors.

use bevy::prelude::*;

use bevy::app::AppExit;
use bevy_tui::prelude::*;

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Layout, Rect};
use tui::style::Style;
use tui::text::Span;
use tui::widgets::{Paragraph, Wrap};
use tui::Frame;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_terminal()?;

    App::new()
        .add_plugins(MinimalTuiPlugins)
        .add_system(run_canvas_ui)
        .add_system(quit_on_esc)
        .run();

    teardown_terminal()?;

    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn quit_on_esc(key_code: Res<Input<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_code.just_pressed(KeyCode::Q) {
        event_writer.send(AppExit);
    }
}

fn render_ui<B: Backend>(
    f: &mut Frame<B>,
) {
    // Render canvas to the entirety of the screen
    let canvas = f.size();

    // Render the color palette
    let palette_rect = Rect {
        x: 2,
        y: 1,
        width: 15,
        height: 11,
    };

    let palette_block = tui::widgets::Block::default()
        .title("Palette")
        .borders(tui::widgets::Borders::ALL)
        .border_style(tui::style::Style::default().fg(tui::style::Color::White))
        .border_type(tui::widgets::BorderType::Rounded);

    f.render_widget(palette_block, palette_rect);
}

#[allow(clippy::needless_pass_by_value)]
fn run_canvas_ui(
    mut terminal: ResMut<bevy_tui::BevyTerminal>,
) {
    terminal
        .0
        .draw(|f| render_ui(f))
        .expect("failed to draw to terminal");
}
