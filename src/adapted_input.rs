use bevy::ecs::event::EventReader;
use bevy::ecs::system::ResMut;
use bevy::input::keyboard::KeyCode;
use bevy::input::{ButtonState, Input};
use bevy::reflect::{FromReflect, Reflect};

// todo: need to add a serialize feature and use it to add the additional serde and bevy reflect
// traits to match bevy_winit.

/// The Bevy version of KeyboardInput requires a scan code which we can't receive from a terminal
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawConsoleEvent(pub crossterm::event::Event);

pub(crate) fn convert_adapted_keyboard_input(
    keyboard_input: &crossterm::event::KeyEvent,
) -> Vec<AdaptedKeyboardInput> {
    let button_state = match convert_input_kind(keyboard_input.kind) {
        Some(state) => state,
        None => {
            return vec![];
        }
    };

    let events: Vec<AdaptedKeyboardInput> = convert_key_code(keyboard_input.code)
        .into_iter()
        .map(|key_code| AdaptedKeyboardInput {
            key_code,
            state: button_state,
        })
        .collect();

    // todo: modifiers need to be separate key inputs

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

fn convert_key_code(key_code: crossterm::event::KeyCode) -> Vec<KeyCode> {
    use crossterm::event::KeyCode::*;

    let mut key_codes = vec![];

    match key_code {
        // what a dumb enum variant name... There is a button dedicated to 'back' as a media key...
        // Why not use the actual name?
        Backspace => key_codes.push(KeyCode::Back),
        Char(ch) => {
            match ch {
                '1' => key_codes.push(KeyCode::Key1),
                '2' => key_codes.push(KeyCode::Key2),
                '3' => key_codes.push(KeyCode::Key3),
                '4' => key_codes.push(KeyCode::Key4),
                '5' => key_codes.push(KeyCode::Key5),
                '6' => key_codes.push(KeyCode::Key6),
                '7' => key_codes.push(KeyCode::Key7),
                '8' => key_codes.push(KeyCode::Key8),
                '9' => key_codes.push(KeyCode::Key9),
                '0' => key_codes.push(KeyCode::Key0),
                'a' => key_codes.push(KeyCode::A),
                'A' => {
                    key_codes.push(KeyCode::A);
                    key_codes.push(KeyCode::LShift);
                }
                // todo: all the typeable keyboard characters...
                _ => {
                    unimplemented!()
                }
            }
        }
        Esc => key_codes.push(KeyCode::Escape),
        F(num) => {
            match num {
                1 => key_codes.push(KeyCode::F1),
                2 => key_codes.push(KeyCode::F2),
                3 => key_codes.push(KeyCode::F3),
                4 => key_codes.push(KeyCode::F4),
                5 => key_codes.push(KeyCode::F5),
                6 => key_codes.push(KeyCode::F6),
                7 => key_codes.push(KeyCode::F7),
                8 => key_codes.push(KeyCode::F8),
                9 => key_codes.push(KeyCode::F9),
                10 => key_codes.push(KeyCode::F10),
                11 => key_codes.push(KeyCode::F11),
                12 => key_codes.push(KeyCode::F12),
                13 => key_codes.push(KeyCode::F13),
                14 => key_codes.push(KeyCode::F14),
                15 => key_codes.push(KeyCode::F15),
                _ => {
                    // do these others actually exist?
                    unimplemented!()
                }
            }
        }
        // todo: all the remaining key codes
        _ => {
            unimplemented!()
        }
    }

    key_codes
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
