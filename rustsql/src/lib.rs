use rusqlite::{params, Connection, Result};

/// Opens a connection to the specified SQLite database.
pub fn open_database(db_file: &str) -> Result<Connection> {
    let connection = Connection::open(db_file)?;
    Ok(connection)
}

/// Creates the users table if it doesn't already exist.
pub fn create_users_table(connection: &Connection) -> Result<()> {
    let create_table_sql = "
        CREATE TABLE IF NOT EXISTS users (
            user_id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            user_age INTEGER NOT NULL
        );
    ";
    connection.execute(create_table_sql, [])?;
    Ok(())
}

/// Inserts a new user into the users table.
pub fn add_user(connection: &Connection, username: &str, user_age: i32) -> Result<()> {
    let insert_sql = "INSERT INTO users (username, user_age) VALUES (?1, ?2);";
    connection.execute(insert_sql, params![username, user_age])?;
    Ok(())
}

/// Fetches all users from the users table.
pub fn fetch_all_users(connection: &Connection) -> Result<Vec<(i32, String, i32)>> {
    let mut stmt = connection.prepare("SELECT * FROM users;")?;
    let users = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    let mut results = Vec::new();
    for user in users {
        results.push(user?);
    }
    Ok(results)
}

/// Updates the age of the specified user.
pub fn modify_user_age(connection: &Connection, user_id: i32, updated_age: i32) -> Result<()> {
    let update_sql = "UPDATE users SET user_age = ?1 WHERE user_id = ?2;";
    connection.execute(update_sql, params![updated_age, user_id])?;
    Ok(())
}

/// Removes the specified user from the users table.
pub fn remove_user(connection: &Connection, user_id: i32) -> Result<()> {
    let delete_sql = "DELETE FROM users WHERE user_id = ?1;";
    connection.execute(delete_sql, params![user_id])?;
    Ok(())
}