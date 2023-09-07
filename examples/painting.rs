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

#[derive(Resource)]
struct BoundedCamera {
    position: [u8; 2],
}

impl Default for BoundedCamera {
    fn default() -> Self {
        Self { position: [127; 2] }
    }
}

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
        .add_systems(Update, (camera_controller, run_canvas_ui, quit_on_esc))
        .run();

    teardown_terminal()?;

    Ok(())
}

// The resource can not be passed by value as we need this signature for Bevy to recognize it as a
// system.
#[allow(clippy::needless_pass_by_value)]
fn camera_controller(key_code: Res<Input<KeyCode>>, mut camera: ResMut<BoundedCamera>) {
    // Up
    if key_code.pressed(KeyCode::K) {
        camera.position[1] = camera.position[1].saturating_sub(1);
    }

    // Down
    if key_code.pressed(KeyCode::J) {
        camera.position[1] = camera.position[1].saturating_add(1);
    }

    // Right
    if key_code.pressed(KeyCode::L) {
        camera.position[0] = camera.position[0].saturating_add(1);
    }

    // Left
    if key_code.pressed(KeyCode::H) {
        camera.position[0] = camera.position[0].saturating_sub(1);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn quit_on_esc(key_code: Res<Input<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_code.just_pressed(KeyCode::Q) {
        event_writer.send(AppExit);
    }
}

fn render_ui<B: Backend>(f: &mut Frame<B>, camera: &BoundedCamera, canvas_data: &CanvasData) {
    // Render canvas to the entirety of the screen
    let canvas = f.size();

    let camera_position_rect = Rect {
        x: 2,
        y: 1,
        width: 18,
        height: 3,
    };

    let camera_position_block = tui::widgets::Block::default()
        .title(" Pos ")
        .borders(tui::widgets::Borders::ALL)
        .border_style(tui::style::Style::default().fg(tui::style::Color::White))
        .border_type(tui::widgets::BorderType::Rounded);

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

    let palette_block = tui::widgets::Block::default()
        .title(" Palette ")
        .borders(tui::widgets::Borders::ALL)
        .border_style(tui::style::Style::default().fg(tui::style::Color::White))
        .border_type(tui::widgets::BorderType::Rounded);

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
