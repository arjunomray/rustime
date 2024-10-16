use crate::cli::Args;
use crate::clock::pomodoro;
use clap::Parser;

mod ascii_art;
mod cli;
mod clock;
mod num_to_ascii;

fn main() {
    let args = Args::parse();
    pomodoro(
        args.session_time,
        args.break_time,
        &args.session_title,
        &args.minimal_version,
    );
}
