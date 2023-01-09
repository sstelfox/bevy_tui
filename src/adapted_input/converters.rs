use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonState;

use crate::adapted_input::AdaptedKeyboardInput;

macro_rules! shifted {
    ($key_code:expr) => {
        vec![KeyCode::LShift, $key_code]
    };
}

macro_rules! unshifted {
    ($key_code:expr) => {
        vec![$key_code]
    };
}

// There is no helping the length of this method as we need to match on all typeable characters.
// There is no complexity or clarity issue resulting from the length so this is allowed.
#[allow(clippy::too_many_lines)]
fn character_key_code(chr: char) -> Vec<KeyCode> {
    match chr {
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
        // todo: all the typeable keyboard characters...
        _ => {
            println!(
                "unknown typable keyboard character: '{chr}' ({})",
                chr as u32
            );
            vec![]
        }
    }
}

pub(super) fn convert_adapted_keyboard_input(
    keyboard_input: &crossterm::event::KeyEvent,
) -> Vec<AdaptedKeyboardInput> {
    let button_state = convert_input_kind(keyboard_input.kind);

    let events: Vec<AdaptedKeyboardInput> = convert_key_code(keyboard_input.code)
        .into_iter()
        .map(|key_code| AdaptedKeyboardInput {
            key_code,
            state: button_state,
        })
        .collect();

    events
}

fn convert_input_kind(kind: crossterm::event::KeyEventKind) -> ButtonState {
    use crossterm::event::KeyEventKind;

    match kind {
        KeyEventKind::Press => ButtonState::Pressed,
        // bevy doesn't have a concept of 'repeat', we do generate fake release events on our
        // though so for our purposes we consider this pressed.
        KeyEventKind::Repeat => ButtonState::Pressed,
        KeyEventKind::Release => ButtonState::Released,
    }
}

fn convert_key_code(key_code: crossterm::event::KeyCode) -> Vec<KeyCode> {
    use crossterm::event::KeyCode as TerminalKeyCode;

    match key_code {
        TerminalKeyCode::Enter => vec![KeyCode::Return],
        TerminalKeyCode::Left => vec![KeyCode::Left],
        TerminalKeyCode::Right => vec![KeyCode::Right],
        TerminalKeyCode::Up => vec![KeyCode::Up],
        TerminalKeyCode::Down => vec![KeyCode::Down],
        TerminalKeyCode::Home => vec![KeyCode::Home],
        TerminalKeyCode::End => vec![KeyCode::End],
        TerminalKeyCode::PageUp => vec![KeyCode::PageUp],
        TerminalKeyCode::PageDown => vec![KeyCode::PageDown],
        TerminalKeyCode::Insert => vec![KeyCode::Insert],
        TerminalKeyCode::Esc => vec![KeyCode::Escape],
        TerminalKeyCode::F(num) => vec![function_key_code(num)],
        // what a dumb enum variant name... There is a button dedicated to 'back' as a media key...
        // Why not use the actual name?
        TerminalKeyCode::Backspace => vec![KeyCode::Back],
        TerminalKeyCode::Tab => unshifted!(KeyCode::Tab),
        TerminalKeyCode::BackTab => shifted!(KeyCode::Tab),
        TerminalKeyCode::Delete => vec![KeyCode::Delete],
        TerminalKeyCode::Char(ch) => character_key_code(ch),
        // The remaining keycodes are not useful to us
        _ => {
            vec![]
        }
    }
}

fn function_key_code(num: u8) -> KeyCode {
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
        13 => KeyCode::F13,
        14 => KeyCode::F14,
        15 => KeyCode::F15,
        16 => KeyCode::F16,
        17 => KeyCode::F17,
        18 => KeyCode::F18,
        19 => KeyCode::F19,
        20 => KeyCode::F20,
        21 => KeyCode::F21,
        22 => KeyCode::F22,
        23 => KeyCode::F23,
        24 => KeyCode::F24,
        _ => {
            // Bevy doesn't support more than this so we're going to assume they don't
            // exist for now. Open an issue if this bites you.
            unreachable!();
        }
    }
}
