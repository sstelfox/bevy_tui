use bevy::input::ButtonState;
use bevy::input::keyboard::KeyCode;
use bevy::reflect::{FromReflect, Reflect};

// todo: need to add a serialize feature and use it to add the additional serde and bevy reflect
// traits to match bevy_winit.

/// The Bevy version of KeyboardInput requires a scan code which we can't receive from a terminal
/// as the code have already been adapted through a keyboard layout long before we receive the
/// event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, FromReflect)]
struct AdaptedKeyboardInput {
    /// The key code of button pressed.
    key_code: KeyCode,

    /// The press state of the key. The release state will only be available on a minor subset of
    /// terminals.
    state: ButtonState,
}

fn convert_adapted_keyboard_input(keyboard_input: crossterm::event::KeyEvent) -> Vec<AdaptedKeyboardInput> {
    let mut events = vec![];

    let button_state = match convert_input_kind(keyboard_input.kind) {
        Some(state) => state,
        None => { return events; },
    };

    events.push(AdaptedKeyboardInput {
        key_code: convert_key_code(keyboard_input.code),
        state: button_state,
    });

    events
}

fn convert_input_kind(kind: crossterm::event::KeyEventKind) -> Option<ButtonState> {
    use crossterm::event::KeyEventKind::*;

    match kind {
        Press => Some(ButtonState::Pressed),
        // bevy doesn't have a concept of 'repeat', so we ignore these events for now
        Repeat => None,
        Release => Some(ButtonState::Released),
    }
}

fn convert_key_code(key_code: crossterm::event::KeyCode) -> KeyCode {
    unimplemented!()
}
