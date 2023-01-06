// The `Reflect` traits makes use of the disallowed `Option#unwrap` method. We can't enforce our
// stricter code requirements on external projects so this is allowed as an exception. What is
// seriously annoying is that this can't be disabled for third-party only code or in code generated
// by the Reflect macro.
#![allow(clippy::disallowed_methods)]

use bevy::app::App;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::ResMut;
use bevy::input::keyboard::KeyCode;
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
pub(crate) struct AdaptedKeyboardInput {
    /// The key code of button pressed.
    key_code: KeyCode,

    /// The press state of the key. The release state will only be available on a minor subset of
    /// terminals.
    state: ButtonState,
}

pub(crate) fn keyboard_input_system(
    mut key_input: ResMut<Input<KeyCode>>,
    mut keyboard_input_events: EventReader<AdaptedKeyboardInput>,
) {
    // We don't get key release events from the terminal. There is an enhancement in the kitty
    // protocol that extends the system to include these but we can't rely on them. This system
    // effectively clears our current key events.
    //
    // todo: in the future I should either detect whether the releases are supported in the
    // terminal or base it off of if I receive a release event.
    key_input.reset_all();

    for event in keyboard_input_events.iter() {
        match event.state {
            ButtonState::Pressed => key_input.press(event.key_code),
            ButtonState::Released => key_input.release(event.key_code),
        }
    }
}

pub(crate) fn event_handler(app: &mut App, event: Event) {
    match event {
        Event::FocusGained | Event::FocusLost => {
            // todo: handle marking us as focused/unfocused in our window equivalent
        }
        Event::Key(event) => {
            converters::convert_adapted_keyboard_input(&event)
                .into_iter()
                .for_each(|ki| app.world.send_event(ki));
        }
        Event::Mouse(event) => {
            // todo: begin handling mouse events
            println!("received mouse event {event:?}\r");
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
