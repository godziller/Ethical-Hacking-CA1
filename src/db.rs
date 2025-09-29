use std::error::Error;

use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::{params, Connection, OptionalExtension};

const DEFAULT_USER: &str = "utz";
const DEFAULT_PIN: &str = "1234";

pub fn init_db(db_path: &str) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(db_path)?;

    // If db table doesnt already exist:
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            username   TEXT PRIMARY KEY,
            pin_hash   TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
        [],
    )?;

    let existing: Option<String> = conn
        .query_row(
            "SELECT username FROM users WHERE username = ?1", params![DEFAULT_USER], |row| row.get(0)
        ).optional()?;

    if existing.is_none() {
        // Hash for newly seeded user
        let hashed = hash(DEFAULT_PIN, DEFAULT_COST)?;
        conn.execute(
            "INSERT INTO users (username, pin_hash) VALUES (?1, ?2)",
            params![DEFAULT_USER, hashed],
        )?;

        println!("Seeded default user `{}` into {}", DEFAULT_USER, db_path);
    }
    Ok(conn)
}

pub fn get_hash_for_user(conn: &Connection, username: &str) -> Result<Option<String>, Box<dyn Error>>{
    let pin_hash: Option<String> = conn.query_row(
        "SELECT pin_hash FROM users WHERE username = ?1",
        params![username],
        |row| row.get(0),
    ).optional()?;
    Ok(pin_hash)
}
