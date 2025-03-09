use crate::cli::Args;
use clap::Parser;

mod ascii_art;
mod cli;
mod clock;
mod db;
mod num_to_ascii;
mod session_manager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let session_info = session_manager::handle_sessions(&args)?;
    session_manager::run_session(
        session_info,
        args.session_time,
        args.break_time,
        &args.minimal_version,
    );
    Ok(())
}
