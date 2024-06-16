# Bevy TUI

A plugin for making interactive Bevy applications with a TUI instead of a graphical interface.

This plugin is mostly usable, my updates to it have been a bit intermittent as my job has taken priority. It's in a fairly solid and usable place though there is a lot of room for improvement and quality of life for developers.

## Quick Start

The default "MinimalPlugins" still includes a large number of graphical plugins which aren't compatible with running in a headless environment. This crate provides an alternate set that work in place of the minimal plugins and primarily gives you access to the Bevy core.

This non-standard minimal core along with the console environment have some trade-offs Bevy developers may not be used to. Please refer to the [Trade Offs](#trade-offs) section for more information.

```rust,no_run
use bevy::prelude::*;
use bevy_tui::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_terminal()?;

    App::new()
        .add_plugins(MinimalTuiPlugins)
        .run();

    teardown_terminal()?;

    Ok(())
}
```

There are a few examples that have been started demonstrating how to make use of portions of this crate.

## Background

This project started as a way to create an administrative client for monitoring the health and status of game servers without adding significant load to them. It occurred to me that properly fleshed out this could be useful in the environment for other projects such as terminal based rogue likes and text adventure games.

## Trade Offs

### Keyboard Events

The keyboard events received from the terminal are not scan codes for a keyboard, nor are they disambiguated between the left / right side of the keyboard. Bevy wants to represent all keyboard interactions using the physical keys which terminals largely don't care about.

To allow for compatibility at some level with existing code bases the keyboard events have been translated into `bevy::input::KeyCode` instances based off a US keyboard layout by hand. Uppercase characters and shifted symbol keys have been translated into two key presses favoring the left hand side of the keyboard. `A` for example becomes two keycodes, `KeyCode::KeyA`, and `KeyCode::ShiftLeft`, similarly `%` becomes `KeyCode::Key5` and `KeyCode::ShiftLeft`. If the raw typed character is desired for something such as text input, it is recommended to instead use the `RawConsoleEvent` type to extract the appropriate typed character. This is also desirable for Unicode code points and non-US keyboard layouts.

There are some keys which are ambiguous on the keyboard and are representable with different combinations of keys, most notably characters on the keypad. In events where a character is both representable shifted over a another character or directly represented by a Bevy enum variant, the shifted version is preferred as it is most likely to have been typed that way on a US keyboard layout.

Due to historical limitations of terminals, not all control sequences or typable characters are allowed, though there are [terminal extensions](https://sw.kovidgoyal.net/kitty/protocol-extensions/) which allow these, they are not commonly or well supported. The events from the terminal the `crossterm` library supports are all represented and exposed to the Bevy event system if your terminal supports it but the default terminal initialization does not attempt to enable these extensions. Not all documented extensions are supported by `crossterm`, please refer to that project for specific support.

If you're using 'Escape' as a key, it will work but you may notice several seconds of delay before it is actually reported. This is also due to an underlying disambiguation of control codes and the key on its own which relies on a timeout under the hood before it gets reported.

### Custom Scheduler

When building terminal applications the scheduler you want to use tends to have different requirements from a graphical based one. The scheduler in here is based off the `winit` equivalent but instead of handling window events, its focused on console events.

This scheduler was built referencing the version available in Bevy 0.12. There have been significant changes to the scheduling systems since then and it is overdue to be refreshed.

## Continuous Integration

There is a tool suite that has the various CI processes bundled up in a repo local tool. I use quite a strict set of options as I've found the recommendations usually assist in future version migrations. A benefit of splitting these out into their own tool is the precise options used can be replicated locally by running the CI processes yourself locally like so:

```console
cargo run -p ci -- default
```
