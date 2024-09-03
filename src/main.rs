use clap::Parser;
use rustime::pomodoro;
use rustime::Args;

fn main() {
    let args = Args::parse();
    pomodoro(args.session_time, args.break_time);
}
