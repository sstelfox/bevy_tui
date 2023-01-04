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

macro_rules! shifted {
    ($key_code:expr) => { vec![KeyCode::LShift, $key_code] }
}

macro_rules! unshifted {
    ($key_code:expr) => { vec![$key_code] }
}

fn convert_key_code(key_code: crossterm::event::KeyCode) -> Vec<KeyCode> {
    use crossterm::event::KeyCode::*;

    let key_codes = match key_code {
        // what a dumb enum variant name... There is a button dedicated to 'back' as a media key...
        // Why not use the actual name?
        Backspace => vec![KeyCode::Back],
        Char(ch) => {
            match ch {
                '`' => unshifted!(KeyCode::Grave),
                '~' => shifted!(KeyCode::Grave),
                '1' => unshifted!(KeyCode::Key1),
                '!' => shifted!(KeyCode::Key1),
                '2' => unshifted!(KeyCode::Key2),
                '@' => shifted!(KeyCode::Key2),
                '3' => unshifted!(KeyCode::Key3),
                '#' => shifted!(KeyCode::Key3),
                '4' => unshifted!(KeyCode::Key4),
                '$' => shifted!(KeyCode::Key4),
                '5' => unshifted!(KeyCode::Key5),
                '%' => shifted!(KeyCode::Key5),
                '6' => unshifted!(KeyCode::Key6),
                '^' => shifted!(KeyCode::Key6),
                '7' => unshifted!(KeyCode::Key7),
                '&' => shifted!(KeyCode::Key7),
                '8' => unshifted!(KeyCode::Key8),
                '*' => shifted!(KeyCode::Key8),
                '9' => unshifted!(KeyCode::Key9),
                '(' => shifted!(KeyCode::Key9),
                '0' => unshifted!(KeyCode::Key0),
                ')' => shifted!(KeyCode::Key0),
                '-' => unshifted!(KeyCode::Minus),
                '_' => shifted!(KeyCode::Minus),
                '=' => unshifted!(KeyCode::Equals),
                // It's worth calling out here that there is a Bevy key code for `+` but it is
                // intended for use with the keypad. I'm prioritizing a more commonly typed
                // variation (this may be a personal logical fallacy but it works for me). If you
                // hit an issue regarding this there is a specific published event with the raw
                // character and I'm open to issues to discuss this further.
                '+' => shifted!(KeyCode::Equals),
                // todo: all the typeable keyboard characters...
                _ => {
                    unimplemented!()
                }
            }
        }
        Esc => vec![KeyCode::Escape],
        F(num) => {
            match num {
                1 => vec![KeyCode::F1],
                2 => vec![KeyCode::F2],
                3 => vec![KeyCode::F3],
                4 => vec![KeyCode::F4],
                5 => vec![KeyCode::F5],
                6 => vec![KeyCode::F6],
                7 => vec![KeyCode::F7],
                8 => vec![KeyCode::F8],
                9 => vec![KeyCode::F9],
                10 => vec![KeyCode::F10],
                11 => vec![KeyCode::F11],
                12 => vec![KeyCode::F12],
                13 => vec![KeyCode::F13],
                14 => vec![KeyCode::F14],
                15 => vec![KeyCode::F15],
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
    };

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
