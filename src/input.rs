// The `Reflect` traits makes use of the disallowed `Option#unwrap` method. We can't enforce our
// stricter code requirements on external projects so this is allowed as an exception. What is
// seriously annoying is that this can't be disabled for third-party only code or in code generated
// by the Reflect macro.
#![allow(clippy::disallowed_methods)]

use bevy::app::App;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::ResMut;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseButton;
use bevy::input::{ButtonState, Input};
use bevy::reflect::{FromReflect, Reflect};
use crossterm::event::Event;

mod converters;

use crate::RawConsoleEvent;

// todo: need to add a serialize feature and use it to add the additional serde and bevy reflect
// traits to match bevy_winit.

/// The Bevy version of `KeyboardInput` requires a scan code which we can't receive from a terminal
/// as the code have already been adapted through a keyboard layout long before we receive the
/// event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, FromReflect)]
pub(crate) struct KeyboardInput {
    /// The key code of button pressed.
    key_code: KeyCode,

    /// The press state of the key. The release state will only be available on a minor subset of
    /// terminals.
    state: ButtonState,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, FromReflect)]
pub(crate) enum MouseInput {
    Button(MouseButton, ButtonState, [u16; 2]),
    Movement([u16; 2]),
}

pub(crate) fn keyboard_input_system(
    mut key_input: ResMut<Input<KeyCode>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    // We don't get key release events from the terminal. There is an enhancement in the kitty
    // protocol that extends the system to include these but we can't rely on them. Instead we
    // attempt to generate our own release events based on whether the key is still pressed.
    key_input.clear();

    let currently_pressed: Vec<KeyCode> = key_input.get_pressed().map(|k| *k).collect();
    let mut pressed_events = vec![];

    for event in keyboard_input_events.iter() {
        match event.state {
            ButtonState::Pressed => {
                pressed_events.push(event.key_code);
                key_input.press(event.key_code);
            },
            ButtonState::Released => key_input.release(event.key_code),
        }
    }

    // TODO: Make release event emulation an option
    // TODO: There is a little bit of a bug that can occur with this, if the keyboard repeat rate
    // delays for the duration of a frame we'll generate a release event before the repeated
    // characters start coming in. I could make this less likely by delaying release events until
    // the end of the next tick or something like that... But it works well enough for now
    for released_key in currently_pressed.into_iter().filter(|kc| !pressed_events.iter().any(|pe| pe == kc)) {
        key_input.release(released_key);
    }
}

pub(crate) fn mouse_input_system(
    mut mouse_input: ResMut<Input<MouseButton>>,
    mut mouse_input_events: EventReader<MouseInput>,
) {
    mouse_input.clear();

    for event in mouse_input_events.iter() {
        let _location = match event {
            MouseInput::Button(_, _, loc) => loc,
            MouseInput::Movement(loc) => loc,
        };

        // todo: generate delta mouse input
        // todo: update current location in mouse_input

        if let MouseInput::Button(btn, state, _) = event {
            match state {
                ButtonState::Pressed => mouse_input.press(*btn),
                ButtonState::Released => mouse_input.release(*btn),
            }
        }
    }
}

pub(crate) fn event_handler(app: &mut App, event: Event) {
    match event {
        Event::FocusGained | Event::FocusLost => {
            // todo: handle marking us as focused/unfocused in our window equivalent
        }
        Event::Key(event) => {
            converters::convert_keyboard_input(&event)
                .into_iter()
                .for_each(|ki| app.world.send_event(ki));
        }
        Event::Mouse(event) => {
            app.world.send_event(converters::convert_mouse_input(&event));
        }
        Event::Paste(ref _data) => {
            // todo: publish event with the pasted content
            // todo: do I get style info?
        }
        Event::Resize(_width, _height) => {
            // todo: update the size of our window equivalent
        }
    }

    app.world.send_event(RawConsoleEvent(event));
}