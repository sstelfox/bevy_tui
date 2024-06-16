//! A more complex example that allows painting some minimal colors.

use bevy::prelude::*;

use bevy::app::AppExit;
use bevy_tui::prelude::*;

use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

#[derive(Resource)]
struct BoundedCamera {
    position: [u8; 2],
}

impl Default for BoundedCamera {
    fn default() -> Self {
        Self { position: [127; 2] }
    }
}

#[allow(dead_code)]
#[derive(Resource)]
struct CanvasData([u8; 256 * 256]);

impl Default for CanvasData {
    fn default() -> Self {
        Self([0; 256 * 256])
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_terminal()?;

    App::new()
        .add_plugins(MinimalTuiPlugins)
        .init_resource::<BoundedCamera>()
        .init_resource::<CanvasData>()
        .add_systems(Update, (camera_controller, run_canvas_ui, quit_system))
        .run();

    teardown_terminal()?;

    Ok(())
}

// The resource can not be passed by value as we need this signature for Bevy to recognize it as a
// system.
#[allow(clippy::needless_pass_by_value)]
fn camera_controller(key_code: Res<ButtonInput<KeyCode>>, mut camera: ResMut<BoundedCamera>) {
    // Up
    if key_code.pressed(KeyCode::KeyK) {
        camera.position[1] = camera.position[1].saturating_sub(1);
    }

    // Down
    if key_code.pressed(KeyCode::KeyJ) {
        camera.position[1] = camera.position[1].saturating_add(1);
    }

    // Right
    if key_code.pressed(KeyCode::KeyL) {
        camera.position[0] = camera.position[0].saturating_add(1);
    }

    // Left
    if key_code.pressed(KeyCode::KeyH) {
        camera.position[0] = camera.position[0].saturating_sub(1);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn quit_system(key_code: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_code.just_pressed(KeyCode::KeyQ) {
        event_writer.send(AppExit::Success);
    }
}

fn render_ui(f: &mut Frame, camera: &BoundedCamera, _canvas_data: &CanvasData) {
    // Render canvas to the entirety of the screen
    let _canvas = f.size();

    let camera_position_rect = Rect {
        x: 2,
        y: 1,
        width: 18,
        height: 3,
    };

    let camera_position_block = ratatui::widgets::Block::default()
        .title(" Pos ")
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .border_type(ratatui::widgets::BorderType::Rounded);

    let camera_position_text_rect = camera_position_block.inner(camera_position_rect);
    f.render_widget(camera_position_block, camera_position_rect);

    let pos_content = Span::styled(
        format!("X: {:3}, Y: {:3}", camera.position[0], camera.position[1]),
        Style::default(),
    );
    let pos_paragraph = Paragraph::new(pos_content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(pos_paragraph, camera_position_text_rect);

    // Render the color palette
    let palette_rect = Rect {
        x: 2,
        y: 5,
        width: 18,
        height: 11,
    };

    let palette_block = ratatui::widgets::Block::default()
        .title(" Palette ")
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .border_type(ratatui::widgets::BorderType::Rounded);

    f.render_widget(palette_block, palette_rect);
}

#[allow(clippy::needless_pass_by_value)]
fn run_canvas_ui(
    mut terminal: ResMut<bevy_tui::BevyTerminal>,
    camera: Res<BoundedCamera>,
    canvas_data: Res<CanvasData>,
) {
    terminal
        .0
        .draw(|f| render_ui(f, &camera, &canvas_data))
        .expect("failed to draw to terminal");
}
