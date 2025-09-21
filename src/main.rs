use crate::{config::cli::Args, db::db::Database, services::discord_presence::DiscordPresence};
use clap::Parser;

mod config;
mod db;
mod services;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;
    let args = Args::parse();
    let session_info = db.handle_sessions(&args)?;

    // Try to initialize Discord presence, but don't panic if it fails
    let mut discord_presence = DiscordPresence::new(
        "1419366249745350666".to_string(),
        session_info.name.as_str(),
        "Getting ready to work",
    );

    // Try to update activity - the function now handles errors internally
    let _ = discord_presence.update_activity();

    db.run_session(
        session_info,
        args.session_time,
        args.break_time,
        &args.minimal_version,
    );
    Ok(())
}
