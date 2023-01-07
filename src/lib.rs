#![feature(rustdoc_missing_doc_code_examples)]

//! A plugin for making interactive Bevy applications with a TUI instead of a graphical interface.
//!
//! # Examples
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_tui::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let _ignored_it = initialize_terminal();
//!
//!     App::new()
//!         .add_plugins(MinimalTuiPlugins);
//!
//!     let _ignored_tt = teardown_terminal();
//!
//!     Ok(())
//! }
//! ```

use bevy::app::{App, CoreStage, Plugin, PluginGroup, PluginGroupBuilder};
use bevy::core::CorePlugin;
use bevy::ecs::system::{Commands, Resource};
use bevy::input::keyboard::KeyCode;
use bevy::input::{ButtonState, Input, InputSystem};
use bevy::prelude::IntoSystemDescriptor;
use bevy::time::TimePlugin;

mod adapted_input;
mod scheduler;
mod terminal_helpers;

/// A quick helper module to allow including all the commonly used and exposed public portions of
/// this library. It can be used in your project like so:
///
/// ```rust
/// use bevy_tui::prelude::*;
/// ```
pub mod prelude {
    pub use crate::terminal_helpers::{initialize_terminal, teardown_terminal};
    pub use crate::{MinimalTuiPlugins, TuiPlugin};
}

use crate::adapted_input::AdaptedKeyboardInput;
use crate::scheduler::{tui_schedule_runner, TuiPersistentState};
use crate::terminal_helpers::create_terminal;

/// The Bevy resource that gets exposed to perform frame render operations. This is a thin wrapper
/// around a [`tui::Terminal`] with no specific backend specified.
///
/// # Examples
///
/// ```
/// use std::io::Write;
///
/// use bevy_tui::Terminal;
///
/// let mut stdout = Vec::new();
/// let mut crossterm_backend = tui::backend::CrosstermBackend::new(stdout);
/// let tui_terminal = tui::Terminal::new(crossterm_backend).unwrap();
///
/// Terminal(tui_terminal);
/// ```
#[derive(Resource)]
pub struct Terminal<T: tui::backend::Backend>(pub tui::Terminal<T>);

/// A short-hand type for a crossterm backed TUI terminal connected to STDOUT. This will likely go
/// away in a more finalized version.
pub type BevyTerminal = Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>;

/// A helper plugin group that sets up the bare minimum plugins for use in a Bevy plugin project.
/// This should be used in place of the Bevy `MinimalPlugins` plugin group as that includes a
/// conflicting `InputPlugin`.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_tui::prelude::*;
///
/// App::new()
///     .add_plugins(MinimalTuiPlugins);
/// ```
pub struct MinimalTuiPlugins;

impl PluginGroup for MinimalTuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin::default())
            .add(TimePlugin::default())
            .add(TuiPlugin::default())
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
/// ```
/// use bevy::prelude::*;
/// use bevy_tui::prelude::*;
///
/// App::new()
///     .add_plugin(TuiPlugin::default());
/// ```
#[derive(Default)]
pub struct TuiPlugin;

impl Plugin for TuiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TuiPersistentState::default())
            .set_runner(tui_schedule_runner)
            .add_startup_system(terminal_setup)
            .add_event::<adapted_input::AdaptedKeyboardInput>()
            .add_event::<RawConsoleEvent>()
            .init_resource::<Input<KeyCode>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                adapted_input::keyboard_input_system.label(InputSystem),
            );

        // Register the common type
        app.register_type::<ButtonState>();

        // Register keyboard types
        app.register_type::<AdaptedKeyboardInput>()
            .register_type::<KeyCode>();
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
/// use bevy_tui::RawConsoleEvent;
///
/// RawConsoleEvent(crossterm::event::Event::FocusGained);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawConsoleEvent(pub crossterm::event::Event);

fn terminal_setup(mut commands: Commands) {
    let term = create_terminal().expect("terminal setup to succeed");
    commands.insert_resource(term);
}
