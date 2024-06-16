// The `Reflect` traits makes use of the disallowed `Option#unwrap` method. We can't enforce our
// stricter code requirements on external projects so this is allowed as an exception. What is
// seriously annoying is that this can't be disabled for third-party only code or in code generated
// by the Reflect macro.
#![allow(clippy::disallowed_methods)]

use bevy::app::App;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::system::{ResMut, Resource};
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::{MouseButton, MouseMotion};
use bevy::input::{ButtonInput, ButtonState};
use bevy::prelude::Event as BevyEvent;
use bevy::reflect::Reflect;
use crossterm::event::Event as CrossEvent;

mod converters;

use crate::RawConsoleEvent;

// todo: need to add a serialize feature and use it to add the additional serde and bevy reflect
// traits to match bevy_winit.

/// The Bevy version of `KeyboardInput` requires a scan code which we can't receive from a terminal
/// as the code have already been adapted through a keyboard layout long before we receive the
/// event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, BevyEvent, Reflect)]
pub(crate) struct KeyboardInput {
    /// The key code of button pressed.
    key_code: KeyCode,

    /// The press state of the key. The release state will only be available on a minor subset of
    /// terminals.
    state: ButtonState,
}

// This enum name triggers one of the pedantic clippy modules which I generally agree with, but in
// this case we're matching the name of the similar data structure in Bevy proper.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, BevyEvent, Reflect)]
pub enum MouseInput {
    Button(MouseButton, ButtonState, [u16; 2]),
    Movement([u16; 2]),
}

/// TODO: write documentation
#[derive(Debug, Default, Reflect, Resource)]
pub struct MouseState {
    last_location: Option<[u16; 2]>,
}
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
/// TODO: write documentation
pub struct WindowResized;

pub(crate) fn keyboard_input_system(
    mut key_input: ResMut<ButtonInput<KeyCode>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    // We don't get key release events from the terminal. There is an enhancement in the kitty
    // protocol that extends the system to include these but we can't rely on them. Instead we
    // attempt to generate our own release events based on whether the key is still pressed.
    key_input.clear();

    // This collect isn't needless as the `key_input.press` happening a little bit further on
    // mutates this state changing what we would receive as a result if we delayed the use of the
    // iterator.
    #[allow(clippy::needless_collect)]
    let currently_pressed: Vec<KeyCode> = key_input.get_pressed().copied().collect();
    let mut pressed_events = vec![];

    for event in keyboard_input_events.read() {
        match event.state {
            ButtonState::Pressed => {
                pressed_events.push(event.key_code);
                key_input.press(event.key_code);
            }
            ButtonState::Released => key_input.release(event.key_code),
        }
    }

    // TODO: Make release event emulation an option
    // TODO: There is a little bit of a bug that can occur with this, if the keyboard repeat rate
    // delays for the duration of a frame we'll generate a release event before the repeated
    // characters start coming in. I could make this less likely by delaying release events until
    // the end of the next tick or something like that... But it works well enough for now
    for released_key in currently_pressed
        .into_iter()
        .filter(|kc| !pressed_events.iter().any(|pe| pe == kc))
    {
        key_input.release(released_key);
    }
}

pub(crate) fn mouse_input_system(
    mut mouse_input: ResMut<ButtonInput<MouseButton>>,
    mut mouse_state: ResMut<MouseState>,
    mut mouse_input_events: EventReader<MouseInput>,
    mut mouse_motion_event_writer: EventWriter<MouseMotion>,
) {
    mouse_input.clear();

    for event in mouse_input_events.read() {
        let new_location = match event {
            MouseInput::Button(_, _, loc) | MouseInput::Movement(loc) => loc,
        };

        if let Some(last_location) = mouse_state.last_location {
            mouse_motion_event_writer.send(MouseMotion {
                delta: bevy::math::Vec2 {
                    x: f32::from(new_location[0]) - f32::from(last_location[0]),
                    y: f32::from(new_location[1]) - f32::from(last_location[1]),
                },
            });
        }

        mouse_state.last_location = Some(*new_location);

        if let MouseInput::Button(btn, state, _) = event {
            match state {
                ButtonState::Pressed => mouse_input.press(*btn),
                ButtonState::Released => mouse_input.release(*btn),
            }
        }
    }
}

pub(crate) fn event_handler(app: &mut App, event: CrossEvent) {
    match event {
        CrossEvent::FocusGained | CrossEvent::FocusLost => {
            // todo: handle marking us as focused/unfocused in our window equivalent
        }
        CrossEvent::Key(event) => {
            for ki in converters::convert_keyboard_input(event) {
                app.world.send_event(ki);
            }
        }
        CrossEvent::Mouse(event) => {
            app.world.send_event(converters::convert_mouse_input(event));
        }
        CrossEvent::Paste(ref _data) => {
            // todo: publish event with the pasted content
            // todo: do I get style info?
        }
        CrossEvent::Resize(_width, _height) => {
            // todo: update the size of our window equivalent
        }
    }

    app.world.send_event(RawConsoleEvent(event));
}
