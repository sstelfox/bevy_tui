#![feature(rustdoc_missing_doc_code_examples)]

//! A plugin for making interactive Bevy applications with a TUI instead of a graphical interface.
//!
//! # Examples
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_tui::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     initialize_terminal()?;
//!
//!     App::new()
//!         .add_plugins(MinimalTuiPlugins)
//!         .run();
//!
//!     teardown_terminal()?;
//!
//!     Ok(())
//! }
//! ```

use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder, PreUpdate, Startup};
use bevy::core::{TaskPoolPlugin, TypeRegistrationPlugin};
use bevy::ecs::system::{Commands, Resource};
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::{MouseButton, MouseMotion};
use bevy::input::{ButtonInput, ButtonState, InputSystem};
use bevy::prelude::{Event, IntoSystemConfigs};
use bevy::time::TimePlugin;

mod error;
mod input;
mod scheduler;
mod terminal_helpers;

pub use error::Error as BevyTuiError;

/// A quick helper module to allow including all the commonly used and exposed public portions of
/// this library. It can be used in your project like so:
///
/// ```
/// use bevy_tui::prelude::*;
/// ```
pub mod prelude {
    pub use crate::input::{MouseState, WindowResized};
    pub use crate::terminal_helpers::{initialize_terminal, teardown_terminal};
    pub use crate::{BevyTuiError, MinimalTuiPlugins, TuiPlugin};
}

use crate::input::{KeyboardInput, MouseInput};
use crate::scheduler::{tui_schedule_runner, TuiPersistentState};
use crate::terminal_helpers::create_terminal;

/// The Bevy resource that gets exposed to perform frame render operations. This is a thin wrapper
/// around a [`ratatui::Terminal`] with no specific backend specified.
///
/// # Examples
///
/// ```no_run
/// # use std::io::Write;
/// use bevy_tui::Terminal;
///
/// let mut stdout = Vec::new();
/// let mut crossterm_backend = ratatui::backend::CrosstermBackend::new(stdout);
/// let tui_terminal = ratatui::Terminal::new(crossterm_backend)?;
///
/// Terminal(tui_terminal);
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Resource)]
pub struct Terminal<T: ratatui::backend::Backend>(pub ratatui::Terminal<T>);

/// A short-hand type for a crossterm backed TUI terminal connected to STDOUT. This will likely go
/// away in a more finalized version.
pub type BevyTerminal = Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

/// A helper plugin group that sets up the bare minimum plugins for use in a Bevy plugin project.
/// This should be used in place of the Bevy `MinimalPlugins` plugin group as that includes a
/// conflicting `InputPlugin`.
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_tui::prelude::*;
///
/// App::new()
///     .add_plugins(MinimalTuiPlugins)
///     .run();
/// ```
pub struct MinimalTuiPlugins;

impl PluginGroup for MinimalTuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin)
            .add(TimePlugin)
            .add(TuiPlugin)
    }
}

/// A Bevy Plugin that includes a dedicated scheduler based on a maximum frame duration and the
/// various events provided to the application from the terminal itself. This should not be used
/// with the standard Bevy `InputPlugin`, the stock `ScheduleRunnerPlugin`, or any of the Winit
/// plugins as they implement and operate on several of the same events as this.
///
/// If you're experiencing issues with `just_pressed` events, missed events, failures to close the
/// application, please first check that these plugins have not been included in the Bevy app.
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_tui::prelude::*;
///
/// App::new()
///     .add_plugins(TuiPlugin)
///     .run();
/// ```
#[derive(Default)]
pub struct TuiPlugin;

impl Plugin for TuiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TuiPersistentState::default())
            .set_runner(tui_schedule_runner)
            .add_systems(Startup, terminal_setup)
            .add_event::<KeyboardInput>()
            .add_event::<RawConsoleEvent>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(PreUpdate, input::keyboard_input_system.in_set(InputSystem))
            .add_event::<MouseInput>()
            .add_event::<MouseMotion>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<input::MouseState>()
            .add_systems(PreUpdate, input::mouse_input_system.in_set(InputSystem));

        // Register the common type
        app.register_type::<ButtonState>();

        // Register keyboard types
        app.register_type::<KeyCode>();

        // Register the mouse types
        app.register_type::<MouseButton>()
            .register_type::<MouseMotion>()
            .register_type::<input::MouseState>();
    }
}

/// A published version of the raw Crossterm events received. This is one of the reasons why this
/// library is currently tied to this particular TUI backend for now. If you're going to be using
/// text input in your UI, these events are likely what you want over the `Input<KeyCode>` events
/// as letter casing and non-US/ASCII keyboard characters are preserved.
///
/// # Examples
///
/// ```
/// # use bevy_tui::RawConsoleEvent;
/// RawConsoleEvent(crossterm::event::Event::FocusGained);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct RawConsoleEvent(pub crossterm::event::Event);

/// Create and register a [`BevyTerminal`] inside the Bevy system for future use by a Terminal UI.
///
/// # Panics
///
/// This method will panic if the underlying [`create_terminal`] function fails to create a
/// terminal likely due to STDOUT being unavailable, or can not be written to.
fn terminal_setup(mut commands: Commands) {
    let term = create_terminal().expect("terminal setup to succeed");
    commands.insert_resource(term);
}
