use bevy::input::keyboard::KeyCode;
// todo: do I get mouse scroll events?
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonState;

use crate::input::{KeyboardInput, MouseInput};

macro_rules! shifted {
    ($key_code:expr) => {
        vec![KeyCode::ShiftLeft, $key_code]
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
        '`' => unshifted!(KeyCode::Backquote),
        '~' => shifted!(KeyCode::Backquote),
        '1' => unshifted!(KeyCode::Digit1),
        '!' => shifted!(KeyCode::Digit1),
        '2' => unshifted!(KeyCode::Digit2),
        '@' => shifted!(KeyCode::Digit2),
        '3' => unshifted!(KeyCode::Digit3),
        '#' => shifted!(KeyCode::Digit3),
        '4' => unshifted!(KeyCode::Digit4),
        '$' => shifted!(KeyCode::Digit4),
        '5' => unshifted!(KeyCode::Digit5),
        '%' => shifted!(KeyCode::Digit5),
        '6' => unshifted!(KeyCode::Digit6),
        // There is a Bevy key code dedicated for this intended for the keypad character.
        // This is instead using the shifted version as that tends to be the more commonly
        // typed character. It may be worth adding the dedicated character keycode as well
        // but so far it hasn't been necessary and may cause unintentional side effects.
        // If you hit an issue regarding this please open an issue to discuss your use case
        // or switch to the published RawConsoleEvent instead.
        '^' => shifted!(KeyCode::Digit6),
        '7' => unshifted!(KeyCode::Digit7),
        '&' => shifted!(KeyCode::Digit7),
        '8' => unshifted!(KeyCode::Digit8),
        '*' => shifted!(KeyCode::Digit8),
        '9' => unshifted!(KeyCode::Digit9),
        '(' => shifted!(KeyCode::Digit9),
        '0' => unshifted!(KeyCode::Digit0),
        ')' => shifted!(KeyCode::Digit0),
        '-' => unshifted!(KeyCode::Minus),
        '_' => shifted!(KeyCode::Minus),
        '=' => unshifted!(KeyCode::Equal),
        // There is a Bevy key code dedicated for this intended for the keypad character.
        // This is instead using the shifted version as that tends to be the more commonly
        // typed character. It may be worth adding the dedicated character keycode as well
        // but so far it hasn't been necessary and may cause unintentional side effects.
        // If you hit an issue regarding this please open an issue to discuss your use case
        // or switch to the published RawConsoleEvent instead.
        '+' => shifted!(KeyCode::Equal),
        'q' => unshifted!(KeyCode::KeyQ),
        'Q' => shifted!(KeyCode::KeyQ),
        'w' => unshifted!(KeyCode::KeyW),
        'W' => shifted!(KeyCode::KeyW),
        'e' => unshifted!(KeyCode::KeyE),
        'E' => shifted!(KeyCode::KeyE),
        'r' => unshifted!(KeyCode::KeyR),
        'R' => shifted!(KeyCode::KeyR),
        't' => unshifted!(KeyCode::KeyT),
        'T' => shifted!(KeyCode::KeyT),
        'y' => unshifted!(KeyCode::KeyY),
        'Y' => shifted!(KeyCode::KeyY),
        'u' => unshifted!(KeyCode::KeyU),
        'U' => shifted!(KeyCode::KeyU),
        'i' => unshifted!(KeyCode::KeyI),
        'I' => shifted!(KeyCode::KeyI),
        'o' => unshifted!(KeyCode::KeyO),
        'O' => shifted!(KeyCode::KeyO),
        'p' => unshifted!(KeyCode::KeyP),
        'P' => shifted!(KeyCode::KeyP),
        '[' => unshifted!(KeyCode::BracketLeft),
        '{' => shifted!(KeyCode::BracketLeft),
        ']' => unshifted!(KeyCode::BracketRight),
        '}' => shifted!(KeyCode::BracketRight),
        '\\' => unshifted!(KeyCode::Backslash),
        '|' => shifted!(KeyCode::Backslash),
        'a' => unshifted!(KeyCode::KeyA),
        'A' => shifted!(KeyCode::KeyA),
        's' => unshifted!(KeyCode::KeyS),
        'S' => shifted!(KeyCode::KeyS),
        'd' => unshifted!(KeyCode::KeyD),
        'D' => shifted!(KeyCode::KeyD),
        'f' => unshifted!(KeyCode::KeyF),
        'F' => shifted!(KeyCode::KeyF),
        'g' => unshifted!(KeyCode::KeyG),
        'G' => shifted!(KeyCode::KeyG),
        'h' => unshifted!(KeyCode::KeyH),
        'H' => shifted!(KeyCode::KeyH),
        'j' => unshifted!(KeyCode::KeyJ),
        'J' => shifted!(KeyCode::KeyJ),
        'k' => unshifted!(KeyCode::KeyK),
        'K' => shifted!(KeyCode::KeyK),
        'l' => unshifted!(KeyCode::KeyL),
        'L' => shifted!(KeyCode::KeyL),
        ';' => unshifted!(KeyCode::Semicolon),
        ':' => shifted!(KeyCode::Semicolon),
        '\'' => unshifted!(KeyCode::Quote),
        '"' => shifted!(KeyCode::Quote),
        'z' => unshifted!(KeyCode::KeyZ),
        'Z' => shifted!(KeyCode::KeyZ),
        'x' => unshifted!(KeyCode::KeyX),
        'X' => shifted!(KeyCode::KeyX),
        'c' => unshifted!(KeyCode::KeyC),
        'C' => shifted!(KeyCode::KeyC),
        'v' => unshifted!(KeyCode::KeyV),
        'V' => shifted!(KeyCode::KeyV),
        'b' => unshifted!(KeyCode::KeyB),
        'B' => shifted!(KeyCode::KeyB),
        'n' => unshifted!(KeyCode::KeyN),
        'N' => shifted!(KeyCode::KeyN),
        'm' => unshifted!(KeyCode::KeyM),
        'M' => shifted!(KeyCode::KeyM),
        ',' => unshifted!(KeyCode::Comma),
        '<' => shifted!(KeyCode::Comma),
        '.' => unshifted!(KeyCode::Period),
        '>' => shifted!(KeyCode::Period),
        '/' => unshifted!(KeyCode::Slash),
        '?' => shifted!(KeyCode::Slash),
        // todo: all the typeable keyboard characters...
        _ => {
            println!(
                "unknown typeable keyboard character: '{chr}' ({})",
                chr as u32
            );
            vec![]
        }
    }
}

pub(super) fn convert_keyboard_input(
    keyboard_input: crossterm::event::KeyEvent,
) -> Vec<KeyboardInput> {
    let button_state = convert_input_kind(keyboard_input.kind);

    let events: Vec<KeyboardInput> = convert_key_code(keyboard_input.code)
        .into_iter()
        .map(|key_code| KeyboardInput {
            key_code,
            state: button_state,
        })
        .collect();

    events
}

pub(super) fn convert_mouse_input(mouse_input: crossterm::event::MouseEvent) -> MouseInput {
    use crossterm::event::MouseEventKind;

    // TODO: I need to convert this to Bevy's coordinate system, maybe I need to do it somewhere
    // else that could get access to the window size?
    let location = [mouse_input.column, mouse_input.row];

    match mouse_input.kind {
        MouseEventKind::Down(btn) | MouseEventKind::Drag(btn) => {
            MouseInput::Button(convert_mouse_button(btn), ButtonState::Pressed, location)
        }
        MouseEventKind::Up(btn) => {
            MouseInput::Button(convert_mouse_button(btn), ButtonState::Released, location)
        }
        MouseEventKind::Moved => MouseInput::Movement(location),
        MouseEventKind::ScrollDown | MouseEventKind::ScrollUp => {
            unimplemented!("{mouse_input:?}\r");
        }
        MouseEventKind::ScrollLeft | MouseEventKind::ScrollRight => {
            unimplemented!("{mouse_input:?}\r");
        }
    }
}

fn convert_mouse_button(button: crossterm::event::MouseButton) -> MouseButton {
    match button {
        crossterm::event::MouseButton::Left => MouseButton::Left,
        crossterm::event::MouseButton::Middle => MouseButton::Middle,
        crossterm::event::MouseButton::Right => MouseButton::Right,
    }
}

fn convert_input_kind(kind: crossterm::event::KeyEventKind) -> ButtonState {
    use crossterm::event::KeyEventKind;

    match kind {
        // bevy doesn't have a concept of 'repeat', we do generate fake release events on our
        // though so for our purposes we consider this pressed.
        KeyEventKind::Press | KeyEventKind::Repeat => ButtonState::Pressed,
        KeyEventKind::Release => ButtonState::Released,
    }
}

fn convert_key_code(key_code: crossterm::event::KeyCode) -> Vec<KeyCode> {
    use crossterm::event::KeyCode as TerminalKeyCode;

    match key_code {
        TerminalKeyCode::Enter => vec![KeyCode::Enter],
        TerminalKeyCode::Left => vec![KeyCode::ArrowLeft],
        TerminalKeyCode::Right => vec![KeyCode::ArrowRight],
        TerminalKeyCode::Up => vec![KeyCode::ArrowUp],
        TerminalKeyCode::Down => vec![KeyCode::ArrowDown],
        TerminalKeyCode::Home => vec![KeyCode::Home],
        TerminalKeyCode::End => vec![KeyCode::End],
        TerminalKeyCode::PageUp => vec![KeyCode::PageUp],
        TerminalKeyCode::PageDown => vec![KeyCode::PageDown],
        TerminalKeyCode::Insert => vec![KeyCode::Insert],
        TerminalKeyCode::Esc => vec![KeyCode::Escape],
        TerminalKeyCode::F(num) => vec![function_key_code(num)],
        // what a dumb enum variant name... There is a button dedicated to 'back' as a media key...
        // Why not use the actual name?
        TerminalKeyCode::Backspace => vec![KeyCode::Backspace],
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
