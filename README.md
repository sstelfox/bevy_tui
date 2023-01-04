# Bevy TUI

A plugin for making interactive Bevy applications with a TUI instead of a graphical interface.

This project started as a way to create an administrative client for monitoring the health and status of game servers without adding significant load to them. It occurred to me that properly fleshed out this could be useful in the environment for other projects such as terminal based roguelikes and text adventure games.

This plugin is currently under heavy and active development and is not quite ready for use.

## Trade Offs

### Keyboard Events

The keyboard events received from the terminal are not scan codes for a keyboard, nor are they disambiguated between the left / right side of the keyboard. Bevy wants to represent all keyboard interactions using the physical keys which terminals largely don't care about.

To allow for compatibility at some level with existing code bases the keyboard events have been translated into `bevy::input::KeyCode` instances based off a US keyboard layout by hand. Uppercase characters and shifted symbol keys have been translated into two key presses favoring the left hand side of the keyboard. `A` for example becomes two keycodes, `KeyCode::A`, and `KeyCode::LShift`, similarly `%` becomes `KeyCode::Key5` and `KeyCode::LShift`. If the raw typed character is desired for something such as text input, it is recommended to instead use the `RawConsoleEvent` type to extract the appropriate typed character. This is also desirable for unicode code points and non-US keyboard layouts.

There are some keys which are ambiguous on the keyboard and are representable with different combinations of keys, most notably characters on the keypad. In events where a character is both representable shifted over a another character or directly represented by a Bevy enum variant, the shifted version is preferred as it is most likely to have been typed that way on a US keyboard layout.

Due to historical limitations of terminals, not all control sequences or typable characters are allowed, though there are [terminal extensions](https://sw.kovidgoyal.net/kitty/protocol-extensions/) which allow these, they are not commonly or well supported. The events from the terminal the `crossterm` library supports are all represented and exposed to the Bevy event system if your terminal supports it but the default terminal initialization does not attempt to enable these extensions. Not all documented extensions are supported by `crossterm`, please refer to that project for specific support.

## Continous Integration

Running all the CI processes locally:

```
cargo run -p ci -- default
```
