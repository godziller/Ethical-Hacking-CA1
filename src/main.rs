use std::io;
use std::io::Write;
use std::error::Error;
use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::{params, Connection, OptionalExtension};
use chrono::{DateTime, Local, Utc};
mod db;


fn login(conn: &Connection, user: &str, pin: &str) -> bool {
    println!("attempting login");
    match db::get_hash_for_user(conn, user){
        Ok(Some(stored_hash)) => match verify(pin, &stored_hash){
            Ok(true) => true,
            _ => false,
        },
        Ok(None) => false,
        Err(_) => false,
    }
}

fn pin_creator(conn: &Connection, user: &str) -> Option<String> {

    for candidate in (0..=9999).map(|n| format!("{:04}", n)) {
        println!("{}", candidate);
        if login(conn, user, &candidate){
            return Some(candidate)
        }
    }  
    None
}
// NOTE change pin to 9999 for all perms

fn main() -> Result<(), Box<dyn Error>> {
    let conn = db::init_db("auth.db")?;
    
    
    println!("User: ");
    io::stdout().flush()?;       // making sure this is consistenetly printed
    
    let mut user = String::new();
    io::stdin().read_line(&mut user)?;
    let user = user.trim();
    
    let dt = Utc::now();
    if let Some(found) = pin_creator(&conn, user){
        let st = Utc::now();
        let elapsed = st-dt;
        println!("Found pin: {}", found);
        println!("Time running: {}ms", elapsed.num_milliseconds())

    }
    else {
        println!("not found");
    }
    Ok(())
    /*
    // Ask for pin:
    let mut pin = rpassword::prompt_password("PIN: ")?;
    let pin = pin.trim();

    if login(&conn ,user, pin) {
        println!("access granted");
    } else {
        println!("access denied")
    }

    Ok(())
    */
}
