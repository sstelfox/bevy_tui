//! A plugin for making interactive Bevy applications with a TUI instead of a graphical interface.

use std::io::Write;
use std::time::{Duration, Instant};

use bevy::app::{App, AppExit, CoreStage, Plugin, PluginGroup, PluginGroupBuilder};
use bevy::core::CorePlugin;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::ecs::system::{Commands, Resource};
use bevy::input::keyboard::KeyCode;
use bevy::input::{ButtonState, Input, InputPlugin, InputSystem};
use bevy::prelude::IntoSystemDescriptor;
use bevy::time::TimePlugin;

use crossterm::event::Event;
use crossterm::event::{poll as poll_term, read as read_term};
use crossterm::QueueableCommand;

use crate::adapted_input::{AdaptedKeyboardInput, RawConsoleEvent};

mod adapted_input;

/// By default the loop will target 4 FPS
const DEFAULT_LOOP_DELAY: Duration = Duration::from_millis(2500);

#[derive(Resource)]
pub struct Terminal<T: tui::backend::Backend>(pub tui::Terminal<T>);

pub type BevyTerminal = Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>;

#[derive(Resource)]
struct TuiPersistentState {
    first_run: bool,
    last_update: Instant,
    timeout_reached: bool,
}

impl TuiPersistentState {
    fn is_first_run(&self) -> bool {
        self.first_run
    }

    fn mark_completed_tick(&mut self) {
        self.first_run = false;
        self.last_update = Instant::now();
    }
}

impl Default for TuiPersistentState {
    fn default() -> Self {
        Self {
            first_run: true,
            last_update: Instant::now(),
            timeout_reached: false,
        }
    }
}

pub struct MinimalTuiPlugins;

impl PluginGroup for MinimalTuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin::default())
            .add(InputPlugin::default())
            .add(TimePlugin::default())
            .add(TuiPlugin::default())
    }
}

#[derive(Default)]
pub struct TuiPlugin;

impl Plugin for TuiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TuiPersistentState::default())
            .set_runner(tui_schedule_runner)
            .add_startup_system(terminal_setup)
            .add_event::<adapted_input::AdaptedKeyboardInput>()
            .add_event::<adapted_input::RawConsoleEvent>()
            .init_resource::<Input<KeyCode>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                adapted_input::keyboard_input_system.label(InputSystem),
            )
            .add_system_to_stage(CoreStage::PostUpdate, adapted_input::keyboard_reset_system);

        // Register the common type
        app.register_type::<ButtonState>();

        // Register keyboard types
        app.register_type::<AdaptedKeyboardInput>()
            .register_type::<KeyCode>();
    }
}

fn event_handler(app: &mut App, event: Event) {
    match event {
        Event::Key(key) => {
            adapted_input::convert_adapted_keyboard_input(&key)
                .into_iter()
                .for_each(|ki| app.world.send_event(ki));
        }
        _ => {
            println!("received unknown event: {event:?}\r");
        }
    }

    app.world.send_event(RawConsoleEvent(event));
}

pub fn initialize_terminal() -> Result<BevyTerminal, Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    stdout.queue(crossterm::terminal::EnterAlternateScreen)?;
    stdout.queue(crossterm::event::EnableBracketedPaste)?;
    stdout.queue(crossterm::event::EnableFocusChange)?;
    stdout.queue(crossterm::event::EnableMouseCapture)?;
    //stdout.queue(crossterm::event::PushKeyboardEnhancementFlags(
    //    crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
    //        | crossterm::event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES
    //        | crossterm::event::KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
    //))?;
    stdout.flush().expect("terminal command trigger");

    let backend = tui::backend::CrosstermBackend::new(stdout);
    let terminal = tui::Terminal::new(backend)?;

    Ok(Terminal(terminal))
}

pub fn teardown_terminal() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::disable_raw_mode()?;

    let mut stdout = std::io::stdout();
    stdout.queue(crossterm::terminal::LeaveAlternateScreen)?;
    stdout.queue(crossterm::event::DisableBracketedPaste)?;
    stdout.queue(crossterm::event::DisableFocusChange)?;
    stdout.queue(crossterm::event::DisableMouseCapture)?;
    //stdout.queue(crossterm::event::PopKeyboardEnhancementFlags)?;
    stdout.queue(crossterm::cursor::Show)?;
    stdout.flush()?;

    Ok(())
}

fn terminal_setup(mut commands: Commands) {
    let term = initialize_terminal().expect("terminal setup to succeed");
    commands.insert_resource(term);
}

fn tick(
    app: &mut App,
    app_exit_event_reader: &mut ManualEventReader<AppExit>,
) -> Result<Option<Duration>, Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    // The app needs to tick once to allow the startup system to setup the terminal. We delay any
    // event processing until this is available otherwise this would become a blocking call until
    // an event is received.
    let first_run = app.world.resource::<TuiPersistentState>().is_first_run();
    if !first_run {
        // todo: need to adjust this delay based on how long the last loop took
        let events_available = poll_term(DEFAULT_LOOP_DELAY)?;

        if events_available {
            // Read all of the available events all at once
            while poll_term(Duration::from_secs(0))? {
                event_handler(app, read_term()?);
            }
        }

        app.world
            .resource_mut::<TuiPersistentState>()
            .timeout_reached = !events_available;
    }

    app.update();
    app.world
        .resource_mut::<TuiPersistentState>()
        .mark_completed_tick();

    if let Some(app_exit_events) = app.world.get_resource::<Events<AppExit>>() {
        if app_exit_event_reader.iter(app_exit_events).last().is_some() {
            return Ok(None);
        }
    }

    Ok(Some(Instant::now() - start_time))
}

fn tui_schedule_runner(mut app: App) {
    let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

    while let Ok(Some(_tick_duration)) = tick(&mut app, &mut app_exit_event_reader) {
        // more stuff to do
    }
}
