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
        Enter => vec![KeyCode::Return],
        Left => vec![KeyCode::Left],
        Right => vec![KeyCode::Right],
        Up => vec![KeyCode::Up],
        Down => vec![KeyCode::Down],
        Home => vec![KeyCode::Home],
        End => vec![KeyCode::End],
        PageUp => vec![KeyCode::PageUp],
        PageDown => vec![KeyCode::PageDown],
        Insert => vec![KeyCode::Insert],
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
                16 => vec![KeyCode::F16],
                17 => vec![KeyCode::F17],
                18 => vec![KeyCode::F18],
                19 => vec![KeyCode::F19],
                20 => vec![KeyCode::F20],
                21 => vec![KeyCode::F21],
                22 => vec![KeyCode::F22],
                23 => vec![KeyCode::F23],
                24 => vec![KeyCode::F24],
                _ => {
                    // Bevy doesn't support more than this so we're going to assume they don't
                    // exist for now. Open an issue if this bites you.
                    unreachable!();
                }
            }
        }
        // what a dumb enum variant name... There is a button dedicated to 'back' as a media key...
        // Why not use the actual name?
        Backspace => vec![KeyCode::Back],
        Tab => unshifted!(KeyCode::Tab),
        BackTab => shifted!(KeyCode::Tab),
        Delete => vec![KeyCode::Delete],
        Char(ch) => {
            match ch {
                ' ' => unshifted!(KeyCode::Space),
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
                // There is a Bevy key code dedicated for this intended for the keypad character.
                // This is instead using the shifted version as that tends to be the more commonly
                // typed character. It may be worth adding the dedicated character keycode as well
                // but so far it hasn't been necessary and may cause unintentional side effects.
                // If you hit an issue regarding this please open an issue to discuss your use case
                // or switch to the published RawConsoleEvent instead.
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
                // There is a Bevy key code dedicated for this intended for the keypad character.
                // This is instead using the shifted version as that tends to be the more commonly
                // typed character. It may be worth adding the dedicated character keycode as well
                // but so far it hasn't been necessary and may cause unintentional side effects.
                // If you hit an issue regarding this please open an issue to discuss your use case
                // or switch to the published RawConsoleEvent instead.
                '+' => shifted!(KeyCode::Equals),
                'q' => unshifted!(KeyCode::Q),
                'Q' => shifted!(KeyCode::Q),
                'w' => unshifted!(KeyCode::W),
                'W' => shifted!(KeyCode::W),
                'e' => unshifted!(KeyCode::E),
                'E' => shifted!(KeyCode::E),
                'r' => unshifted!(KeyCode::R),
                'R' => shifted!(KeyCode::R),
                't' => unshifted!(KeyCode::T),
                'T' => shifted!(KeyCode::T),
                'y' => unshifted!(KeyCode::Y),
                'Y' => shifted!(KeyCode::Y),
                'u' => unshifted!(KeyCode::U),
                'U' => shifted!(KeyCode::U),
                'i' => unshifted!(KeyCode::I),
                'I' => shifted!(KeyCode::I),
                'o' => unshifted!(KeyCode::O),
                'O' => shifted!(KeyCode::O),
                'p' => unshifted!(KeyCode::P),
                'P' => shifted!(KeyCode::P),
                '[' => unshifted!(KeyCode::LBracket),
                '{' => shifted!(KeyCode::LBracket),
                ']' => unshifted!(KeyCode::RBracket),
                '}' => shifted!(KeyCode::RBracket),
                '\\' => unshifted!(KeyCode::Backslash),
                '|' => shifted!(KeyCode::Backslash),
                'a' => unshifted!(KeyCode::A),
                'A' => shifted!(KeyCode::A),
                's' => unshifted!(KeyCode::S),
                'S' => shifted!(KeyCode::S),
                'd' => unshifted!(KeyCode::D),
                'D' => shifted!(KeyCode::D),
                'f' => unshifted!(KeyCode::F),
                'F' => shifted!(KeyCode::F),
                'g' => unshifted!(KeyCode::G),
                'G' => shifted!(KeyCode::G),
                'h' => unshifted!(KeyCode::H),
                'H' => shifted!(KeyCode::H),
                'j' => unshifted!(KeyCode::J),
                'J' => shifted!(KeyCode::J),
                'k' => unshifted!(KeyCode::K),
                'K' => shifted!(KeyCode::K),
                'l' => unshifted!(KeyCode::L),
                'L' => shifted!(KeyCode::L),
                ';' => unshifted!(KeyCode::Semicolon),
                ':' => shifted!(KeyCode::Semicolon),
                '\'' => unshifted!(KeyCode::Apostrophe),
                '"' => shifted!(KeyCode::Apostrophe),
                'z' => unshifted!(KeyCode::Z),
                'Z' => shifted!(KeyCode::Z),
                'x' => unshifted!(KeyCode::X),
                'X' => shifted!(KeyCode::X),
                'c' => unshifted!(KeyCode::C),
                'C' => shifted!(KeyCode::C),
                'v' => unshifted!(KeyCode::V),
                'V' => shifted!(KeyCode::V),
                'b' => unshifted!(KeyCode::B),
                'B' => shifted!(KeyCode::B),
                'n' => unshifted!(KeyCode::N),
                'N' => shifted!(KeyCode::N),
                'm' => unshifted!(KeyCode::M),
                'M' => shifted!(KeyCode::M),
                ',' => unshifted!(KeyCode::Comma),
                '<' => shifted!(KeyCode::Comma),
                '.' => unshifted!(KeyCode::Period),
                '>' => shifted!(KeyCode::Period),
                '/' => unshifted!(KeyCode::Slash),
                '?' => shifted!(KeyCode::Slash),
                //'' => unshifted!(KeyCode::),
                //'' => shifted!(KeyCode::),
                // todo: all the typeable keyboard characters...
                _ => {
                    println!("unknown typable keyboard character: {key_code:?}");
                    vec![]
                }
            }
        },
        // todo: all the remaining key codes
        _ => {
            println!("unknown event: {key_code:?}");
            vec![]
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

pub(crate) fn event_handler(app: &mut App, event: Event) {
    match event {
        Event::FocusGained => {
            // todo: handle marking us as actively focused in our window equivalent
        },
        Event::FocusLost => {
            // todo: handle marking us as no longer focused in our window equivalent
        },
        Event::Key(event) => {
            adapted_input::convert_adapted_keyboard_input(&event)
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
