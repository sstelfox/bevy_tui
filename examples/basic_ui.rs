//! Basic TUI rendering example for Bevy

use bevy::prelude::*;

use bevy::app::AppExit;
use bevy_tui::MinimalTuiPlugins;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(MinimalTuiPlugins)
        //.add_system(run_basic_ui)
        .add_system(quit_on_esc)
        .run();

    // The TuiPlugin handles the initialization of the terminal itself, as there aren't "shutdown"
    // systems in Bevy this needs to be handled manually.
    bevy_tui::teardown_terminal()?;

    Ok(())
}

fn quit_on_esc(key_code: Res<Input<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    println!("quit on esc keycode state: {key_code:?}\r");

    if key_code.just_pressed(KeyCode::Escape) {
        println!("sent exit event\r");
        event_writer.send(AppExit);
    }
}

//fn run_basic_ui(mut terminal: ResMut<bevy_tui::BevyTerminal>) {
//    terminal
//        .0
//        .draw(|f| render_ui(f))
//        .expect("failed to draw to terminal");
//}

//fn render_ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>) {
//    let chunks = tui::layout::Layout::default()
//        .constraints([tui::layout::Constraint::Percentage(100)].as_ref())
//        .split(f.size());
//
//    let styled_content = tui::text::Span::styled("Hello Bevy!", tui::style::Style::default());
//    let paragraph = tui::widgets::Paragraph::new(styled_content)
//        .alignment(tui::layout::Alignment::Center)
//        .wrap(tui::widgets::Wrap { trim: true });
//
//    f.render_widget(paragraph, chunks[0]);
//}
