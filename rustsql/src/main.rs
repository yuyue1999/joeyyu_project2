// src/main.rs

use rustsql::*; // This line imports all functions from lib.rs
use rusqlite::Result; // Import the rusqlite Result type

fn main() -> Result<()> {
    let connection = open_database("database.db")?;
    create_users_table(&connection)?;

    // Example usage
    add_user(&connection, "Alice", 30)?;
    add_user(&connection, "Bob", 25)?;

    let users = fetch_all_users(&connection)?;
    for (user_id, username, user_age) in users {
        println!("User ID: {}, Username: {}, Age: {}", user_id, username, user_age);
    }

    modify_user_age(&connection, 1, 35)?;
    remove_user(&connection, 2)?;

    Ok(())
}
