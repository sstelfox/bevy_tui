//! A plugin for making interactive Bevy applications with a TUI instead of a graphical interface.

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

pub mod prelude {
    pub use crate::terminal_helpers::{initialize_terminal, teardown_terminal};
    pub use crate::MinimalTuiPlugins;
}

use crate::adapted_input::AdaptedKeyboardInput;
use crate::scheduler::{tui_schedule_runner, TuiPersistentState};
use crate::terminal_helpers::create_terminal;

#[derive(Resource)]
pub struct Terminal<T: tui::backend::Backend>(pub tui::Terminal<T>);

pub type BevyTerminal = Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>;

pub struct MinimalTuiPlugins;

impl PluginGroup for MinimalTuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin::default())
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
            );

        // Register the common type
        app.register_type::<ButtonState>();

        // Register keyboard types
        app.register_type::<AdaptedKeyboardInput>()
            .register_type::<KeyCode>();
    }
}

fn terminal_setup(mut commands: Commands) {
    let term = create_terminal().expect("terminal setup to succeed");
    commands.insert_resource(term);
}
