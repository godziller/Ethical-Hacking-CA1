use std::io;
use std::io::Write;
use std::error::Error;
use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::{params, Connection, OptionalExtension};
mod db;


fn login(conn: &Connection, user: &str, pin: &str) -> bool {
    match db::get_hash_for_user(conn, user){
        Ok(Some(stored_hash)) => match verify(pin, &stored_hash){
            Ok(true) => true,
            _ => false,
        },
        Ok(None) => false,
        Err(_) => false,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = db::init_db("auth.db")?;
    // Ask for user name 
    println!("User: ");
    io::stdout().flush()?;       // making sure this is consistenetly printed
    
    let mut user = String::new();
    io::stdin().read_line(&mut user)?;
    let user = user.trim();

    // Ask for pin:
    let mut pin = rpassword::prompt_password("PIN: ")?;
    let pin = pin.trim();

    if login(&conn ,user, pin) {
        println!("access granted");
    } else {
        println!("access denied")
    }

    Ok(())
}
