//! A plugin for making interactive Bevy applications with a TUI instead of a graphical interface.

use std::io::Write;
use std::time::{Duration, Instant};

use bevy::app::{App, AppExit, CoreStage, Plugin, PluginGroup, PluginGroupBuilder};
use bevy::core::CorePlugin;
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::ecs::system::{Commands, Resource};
use bevy::input::keyboard::KeyCode;
use bevy::input::{ButtonState, Input, InputSystem};
use bevy::prelude::IntoSystemDescriptor;
use bevy::time::TimePlugin;

use crossterm::event::Event;
use crossterm::event::{poll as poll_term, read as read_term};
use crossterm::QueueableCommand;

use crate::adapted_input::{AdaptedKeyboardInput, RawConsoleEvent};

mod adapted_input;
mod scheduler;
mod terminal_helpers;

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
