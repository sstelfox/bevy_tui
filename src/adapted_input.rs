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

pub(crate) fn convert_adapted_keyboard_input(
    keyboard_input: crossterm::event::KeyEvent,
) -> Vec<AdaptedKeyboardInput> {
    let mut events = vec![];

    let button_state = match convert_input_kind(keyboard_input.kind) {
        Some(state) => state,
        None => {
            println!("no valid input kind\r");
            return events;
        }
    };

    events.push(AdaptedKeyboardInput {
        key_code: convert_key_code(keyboard_input.code),
        state: button_state,
    });

    // todo: modifiers need to be separate key inputs

    println!("{events:?}\r");

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
    use crossterm::event::KeyCode::*;

    match key_code {
        // what a dumb enum variant name... There is a button dedicated to 'back' as a media key...
        // Why not use the actual name?
        Backspace => KeyCode::Back,
        Char(ch) => {
            match ch {
                '1' => KeyCode::Key1,
                '2' => KeyCode::Key2,
                '3' => KeyCode::Key3,
                '4' => KeyCode::Key4,
                '5' => KeyCode::Key5,
                '6' => KeyCode::Key6,
                '7' => KeyCode::Key7,
                '8' => KeyCode::Key8,
                '9' => KeyCode::Key9,
                '0' => KeyCode::Key0,
                // todo: all the typeable keyboard characters...
                _ => {
                    unimplemented!()
                }
            }
        }
        Esc => KeyCode::Escape,
        F(num) => {
            match num {
                1 => KeyCode::F1,
                2 => KeyCode::F2,
                3 => KeyCode::F3,
                4 => KeyCode::F4,
                5 => KeyCode::F5,
                6 => KeyCode::F6,
                7 => KeyCode::F7,
                8 => KeyCode::F8,
                9 => KeyCode::F9,
                10 => KeyCode::F10,
                11 => KeyCode::F11,
                12 => KeyCode::F12,
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
}

pub(crate) fn keyboard_input_system(
    mut key_input: ResMut<Input<KeyCode>>,
    mut keyboard_input_events: EventReader<AdaptedKeyboardInput>,
) {
    println!("initial keyboard state: {key_input:?}\r");

    for event in keyboard_input_events.iter() {
        println!("pre-event keyboard state: {event:?} -> {key_input:?}\r");

        match event.state {
            ButtonState::Pressed => key_input.press(event.key_code),
            ButtonState::Released => key_input.release(event.key_code),
        }

        println!("post-event keyboard state: {event:?} -> {key_input:?}\r");
    }
}

// We don't get key release events from the terminal. There is an enhancement in the kitty protocol
// that extends the system to include these but we can't rely on them. This system effectively
// clears our current key events.
//
// todo: in the future I should either detect whether the releases are supported in the terminal or
// base it off of if I receive a release event.
pub(crate) fn keyboard_reset_system(mut key_input: ResMut<Input<KeyCode>>) {
    println!("reset system state: {key_input:?}\r");
    key_input.reset_all();
    println!("post reset system state: {key_input:?}\r");
}
