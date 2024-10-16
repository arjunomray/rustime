# Rustime

A simple to use cli based pomodoro timer.

## Installation

1. Install rust

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

2. Use cargo to install

  ```sh
  cargo install rustime
  ```

## Usage commands

```sh
->rustime -h
Usage: rustime [OPTIONS]

Options:
  -s, --session-time <SESSION_TIME>    [default: 25]
  -b, --break-time <BREAK_TIME>        [default: 5]
  -n, --session-title <SESSION_TITLE>  [default: "Working on Stuff"]
  -m, --minimal-version
  -h, --help                           Print help
```
