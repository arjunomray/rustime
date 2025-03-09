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
  -t, --session-title <SESSION_TITLE>
  -m, --minimal-version
  -l, --list-sessions
  -n, --new-session <NEW_SESSION>
  -h, --help                           Print help
```
