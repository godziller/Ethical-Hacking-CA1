use std::error::Error;

use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use rusqlite::{params, Connection, OptionalExtension};
use chrono::{Utc, DateTime};

const DEFAULT_USER: &str = "utz";
const DEFAULT_PIN: &str = "0001";

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
        //let hashed = hash(DEFAULT_PIN, DEFAULT_COST)?;
        let hashed = hash_iterative(2)
            .map_err(|e| Box::<dyn Error>::from(e))?;

        conn.execute(
            "INSERT INTO users (username, pin_hash) VALUES (?1, ?2)",
            params![DEFAULT_USER, hashed],
        )?;

        println!("Seeded default user `{}` into {}", DEFAULT_USER, db_path);
    }
    Ok(conn)
}

pub fn hash_iterative(times: usize) -> Result<String, BcryptError> {
    // Start from the plain PIN
    let mut current = DEFAULT_PIN.to_string();
    let st = Utc::now();
    for _ in 0..times {
        // hash the bytes of `current` and replace it with the encoded hash
        println!("hashing!");
        current = hash(&current, DEFAULT_COST)?;
    }
    let et = Utc::now();
    let elapsed = et-st;
    let elapsed = elapsed.num_milliseconds();
    println!("Time spent Hashing: {}", elapsed);
    Ok(current)
}
pub fn get_hash_for_user(conn: &Connection, username: &str, pin: &str) -> Result<Option<String>, BcryptError> {
    let mut current = pin;
    let pin_hash = hash_iterative(2)?;
    Ok(Some(pin_hash))

    /*
    let pin_hash: Option<String> = conn.query_row(
        "SELECT pin_hash FROM users WHERE username = ?1",
        params![username],
        |row| row.get(0),
    ).optional()?;
    Ok(pin_hash)
    */

}
