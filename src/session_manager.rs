use crate::{cli::Args, db};
use std::io::{self, Write};
use std::process;
pub struct SessionInfo {
    pub name: String,
    pub id: i64,
}

pub fn handle_sessions(args: &Args) -> Result<SessionInfo, Box<dyn std::error::Error>> {
    db::init_db()?;

    if args.list_sessions {
        list_all_sessions()?;
        process::exit(0);
    }

    if let Some(new_session_name) = &args.new_session {
        return create_new_session(new_session_name);
    }

    if let Some(session_title) = &args.session_title {
        return handle_specified_session(session_title);
    }

    handle_session_selection()
}

fn list_all_sessions() -> Result<(), Box<dyn std::error::Error>> {
    let sessions = db::get_all_sessions()?;
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

fn create_new_session(name: &str) -> Result<SessionInfo, Box<dyn std::error::Error>> {
    match db::create_session(name) {
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

fn handle_specified_session(name: &str) -> Result<SessionInfo, Box<dyn std::error::Error>> {
    match db::get_session_by_name(name) {
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
                create_new_session(name)
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

fn handle_session_selection() -> Result<SessionInfo, Box<dyn std::error::Error>> {
    let sessions = db::get_all_sessions()?;
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
        create_new_session(new_name)
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
            create_new_session(new_name)
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
    session_info: SessionInfo,
    session_time: i32,
    break_time: i32,
    minimal_version: &bool,
) {
    let session_id = session_info.id;
    let session_name = session_info.name;

    pomodoro_with_db(
        session_time,
        break_time,
        &session_name,
        minimal_version,
        session_id,
    );
}

fn pomodoro_with_db(
    session_time: i32,
    break_time: i32,
    session_name: &str,
    minimal_version: &bool,
    session_id: i64,
) {
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
                    crate::clock::countdown(
                        session_time,
                        crate::clock::SessionType::SESSION,
                        &session_name.to_string(),
                        session_counter,
                        minimal_version,
                    );
                    session_counter += 1;
                    if let Err(e) = db::increment_session_count(session_id) {
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
                    crate::clock::countdown(
                        break_time,
                        crate::clock::SessionType::BREAK,
                        &session_name.to_string(),
                        session_counter,
                        minimal_version,
                    );
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
