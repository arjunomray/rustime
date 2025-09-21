use crate::{config::cli::Args, db::db::Database, services::discord_presence::DiscordPresence};
use clap::Parser;

mod config;
mod db;
mod services;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;
    let args = Args::parse();
    let session_info = db.handle_sessions(&args)?;
    let mut client = DiscordPresence::new(
        "1419366249745350666".to_string(),
        session_info.name.as_str(),
        "Getting ready to work",
    );
    match client.update_activity() {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to update Discord activity: {}", e),
    }

    db.run_session(
        session_info,
        args.session_time,
        args.break_time,
        &args.minimal_version,
    );
    Ok(())
}
