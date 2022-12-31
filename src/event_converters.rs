use bevy::input::keyboard::{KeyCode, KeyboardInput};
use crossterm::event::KeyEvent;

pub(crate) fn crossterm_keycode(term_input: KeyEvent) -> KeyCode {
    println!("{term_input:?}");
    unimplemented!()
    //BevyKeyCode {
    //    scan_code: keycode_to_scancode(&term_input),
    //    state: ButtonState::Pressed,
    //    key_code: None,
    //}
}
