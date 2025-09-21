use rusqlite::{Connection, Result};

use crate::config::cli::Args;
use crate::services::clock::{Clock, SessionType};
use std::io::{self, Write};
use std::path::Path;
use std::process;

pub struct SessionInfo {
    pub name: String,
    pub id: i64,
}

pub struct SessionRecord {
    pub id: i64,
    pub name: String,
    pub completed_count: i32,
}

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = get_db_path();
        if let Some(parent) = Path::new(&db_path).parent() {
            std::fs::create_dir_all(parent).ok();
        };

        let conn = Connection::open(&db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            completed_count INTEGER NOT NULL DEFAULT 0
        )",
            [],
        )?;
        Ok(Database { conn })
    }

    pub fn get_all_sessions(&self) -> Result<Vec<SessionRecord>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, completed_count FROM sessions ORDER BY name")?;

        let session_iter = stmt.query_map([], |row| {
            Ok(SessionRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                completed_count: row.get(2)?,
            })
        })?;

        let mut sessions = Vec::new();
        for session in session_iter {
            sessions.push(session?);
        }

        Ok(sessions)
    }

    pub fn create_session(&self, name: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO sessions (name, completed_count) VALUES (?1, 0)",
            [name],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_session_by_name(&self, name: &str) -> Result<Option<SessionRecord>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, completed_count FROM sessions WHERE name = ?1")?;

        let mut rows = stmt.query([name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(SessionRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                completed_count: row.get(2)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn increment_session_count(&self, session_id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE sessions SET completed_count = completed_count + 1 WHERE id = ?1",
            [session_id],
        )?;

        Ok(())
    }

    pub fn handle_sessions(
        self: &Database,
        args: &Args,
    ) -> Result<SessionInfo, Box<dyn std::error::Error>> {
        if args.list_sessions {
            self.list_all_sessions()?;
            process::exit(0);
        }

        if let Some(new_session_name) = &args.new_session {
            return self.create_new_session(new_session_name);
        }

        if let Some(session_title) = &args.session_title {
            return self.handle_specified_session(session_title);
        }

        self.handle_session_selection()
    }

    fn list_all_sessions(self: &Database) -> Result<(), Box<dyn std::error::Error>> {
        let sessions = self.get_all_sessions()?;
        if sessions.is_empty() {
            println!("No sessions found. Create one with --new-session");
            return Ok(());
        }
        println!("Available sessions");
        for (i, session) in sessions.iter().enumerate() {
            println!(
                "{}. {} (completed {} times)",
                i + 1,
                session.name,
                session.completed_count
            );
        }
        Ok(())
    }

    fn create_new_session(
        self: &Database,
        name: &str,
    ) -> Result<SessionInfo, Box<dyn std::error::Error>> {
        match self.create_session(name) {
            Ok(id) => {
                println!("Created new session: {}", name);
                Ok(SessionInfo {
                    name: name.to_string(),
                    id,
                })
            }
            Err(e) => {
                eprintln!("Failed to create session: {}", e);
                Err(e.into())
            }
        }
    }

    fn handle_specified_session(
        self: &Database,
        name: &str,
    ) -> Result<SessionInfo, Box<dyn std::error::Error>> {
        match self.get_session_by_name(name) {
            Ok(Some(session)) => {
                println!(
                    "Using existing session: {} (completed {} times)",
                    session.name, session.completed_count
                );
                Ok(SessionInfo {
                    name: session.name,
                    id: session.id,
                })
            }
            Ok(None) => {
                print!("Session '{}' doesn't exist. Create it? (Y/n): ", name);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if input.trim().to_lowercase() == "y"
                    || input.trim().to_lowercase() == "yes"
                    || input.trim().is_empty()
                {
                    self.create_new_session(name)
                } else {
                    println!("Session creation cancelled");
                    process::exit(0);
                }
            }
            Err(e) => {
                eprintln!("Database error: {}", e);
                Err(e.into())
            }
        }
    }

    fn handle_session_selection(
        self: &Database,
    ) -> Result<SessionInfo, Box<dyn std::error::Error>> {
        let sessions = self.get_all_sessions()?;
        if sessions.is_empty() {
            println!("No sessions found. Please create one:");
            print!("Enter session name: ");
            io::stdout().flush()?;

            let mut new_name = String::new();
            io::stdin().read_line(&mut new_name)?;
            let new_name = new_name.trim();

            if new_name.is_empty() {
                println!("Session name cannot be empty.");
                process::exit(1);
            }
            self.create_new_session(new_name)
        } else {
            println!("Select a session or enter n to create a new one:");
            for (i, session) in sessions.iter().enumerate() {
                println!(
                    "{}. {} (completed {} times )",
                    i + 1,
                    session.name,
                    session.completed_count
                );
            }

            print!("Selection: ");
            io::stdout().flush()?;

            let mut selection = String::new();
            io::stdin().read_line(&mut selection)?;
            let selection = selection.trim();

            if selection.to_ascii_lowercase() == "n" {
                print!("Enter new session name:");
                io::stdout().flush()?;

                let mut new_name = String::new();
                io::stdin().read_line(&mut new_name)?;
                let new_name = new_name.trim();
                if new_name.is_empty() {
                    println!("Session name cannot be empty");
                    process::exit(1);
                }
                self.create_new_session(new_name)
            } else if let Ok(index) = selection.parse::<usize>() {
                if index > 0 && index <= sessions.len() {
                    let selected_session = &sessions[index - 1];
                    Ok(SessionInfo {
                        name: selected_session.name.clone(),
                        id: selected_session.id,
                    })
                } else {
                    println!("Invalid selection");
                    process::exit(1);
                }
            } else {
                println!("Invalid selection");
                process::exit(1);
            }
        }
    }

    pub fn run_session(
        self: &Database,
        session_info: SessionInfo,
        session_time: i32,
        break_time: i32,
        minimal_version: &bool,
    ) {
        let clock = Clock::new(&session_info, session_time, break_time, minimal_version);

        self.pomodoro_with_db(clock);
    }

    fn pomodoro_with_db(self: &Database, clock: Clock) {
        let mut session_counter = 0;
        loop {
            loop {
                println!("Start session? (Y/n)");
                let mut session_check = String::new();
                io::stdin()
                    .read_line(&mut session_check)
                    .expect("Failed to read");
                let session_check = session_check.trim().to_lowercase();

                match session_check.as_str() {
                    "y" | "yes" | "" => {
                        clock.countdown(SessionType::SESSION, session_counter);
                        session_counter += 1;
                        if let Err(e) = self.increment_session_count(clock.session_info.id) {
                            eprintln!("Failed to update session count: {}", e);
                        }
                        break;
                    }
                    "n" | "no" => {
                        println!("Exiting program... worked for {session_counter} session(s)");
                        process::exit(0);
                    }
                    _ => {
                        println!("Invalid Input");
                        continue;
                    }
                }
            }
            loop {
                println!("Start break? (Y/n)");
                let mut break_check = String::new();
                io::stdin()
                    .read_line(&mut break_check)
                    .expect("Failed to read");

                let break_check = break_check.trim().to_lowercase();

                match break_check.as_str() {
                    "y" | "yes" => {
                        clock.countdown(SessionType::BREAK, session_counter);
                        break;
                    }

                    "n" | "no" => {
                        println!("Exiting program... worked for {session_counter} sessions");
                        process::exit(0)
                    }
                    _ => {
                        println!("Invalid input.");
                        continue;
                    }
                }
            }
        }
    }
}

fn get_db_path() -> String {
    let home = dirs::home_dir().unwrap_or_default();
    home.join(".config/rustime/sessions.db")
        .to_str()
        .unwrap_or("./.rustime.db")
        .to_string()
}
