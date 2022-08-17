# Pomodoro TUI

The [Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique) is a time management method that breaks work into intervals of working, small breaks in between and a long break every four cycles.
This program is a minimal Terminal-UI application that counts down the segments and flashes red once finished, waiting for the user to manually advance to the next segment.
Notifications using the notification daemon via `notify-send` can be enabled using the `-n` flag.

## Usage
```sh
git clone https://github.com/Pyxels/pomodoro-tui
cd pomodoro-tui
cargo build --release
./target/release/pomodoro-tui
```
For help, run `pomodoro-tui --help`.

### Options

| Option                    | Description                                              | Default |
|---------------------------|----------------------------------------------------------|---------|
| --help                    | Print help information                                   |         |
| --version                 | Print version information                                |         |
| --notifications           | Enable desktop notifications via the notification daemon |         |
| --work <WORK>             | Duration of the work segment (minutes)                   | 25      |
| --small-rest <SMALL_REST> | Duration of the short rest segment (minutes)             | 5       |
| --large-rest <LARGE_REST> | Duration of the large rest segment (minutes)             | 35      |


