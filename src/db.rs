use rusqlite::{Connection, Result};
use std::path::Path;

pub struct SessionRecord {
    pub id: i64,
    pub name: String,
    pub completed_count: i32,
}

pub fn init_db() -> Result<Connection> {
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
    Ok(conn)
}

pub fn get_db_path() -> String {
    let home = dirs::home_dir().unwrap_or_default();
    home.join(".config/rustime/sessions.db")
        .to_str()
        .unwrap_or("./.rustime.db")
        .to_string()
}

pub fn get_all_sessions() -> Result<Vec<SessionRecord>> {
    let conn = init_db()?;
    let mut stmt = conn.prepare("SELECT id, name, completed_count FROM sessions ORDER BY name")?;

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

pub fn create_session(name: &str) -> Result<i64> {
    let conn = init_db()?;
    conn.execute(
        "INSERT INTO sessions (name, completed_count) VALUES (?1, 0)",
        [name],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_session_by_name(name: &str) -> Result<Option<SessionRecord>> {
    let conn = init_db()?;
    let mut stmt =
        conn.prepare("SELECT id, name, completed_count FROM sessions WHERE name = ?1")?;

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

pub fn increment_session_count(session_id: i64) -> Result<()> {
    let conn = init_db()?;
    conn.execute(
        "UPDATE sessions SET completed_count = completed_count + 1 WHERE id = ?1",
        [session_id],
    )?;

    Ok(())
}
