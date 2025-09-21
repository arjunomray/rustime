use crate::{config::cli::Args, db::db::Database};
use clap::Parser;

mod config;
mod db;
mod services;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;
    let args = Args::parse();

    let session_info = db.handle_sessions(&args)?;
    db.run_session(
        session_info,
        args.session_time,
        args.break_time,
        &args.minimal_version,
    );
    Ok(())
}
