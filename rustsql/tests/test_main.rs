#[cfg(test)]
mod tests {
    use rustsql::{add_user, create_users_table, fetch_all_users, modify_user_age, open_database, remove_user};
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        // Create an in-memory SQLite database for testing purposes
        let connection = Connection::open_in_memory().expect("Failed to open in-memory database");
        create_users_table(&connection).expect("Failed to create users table");
        connection
    }

    #[test]
    fn test_add_user() {
        let connection = setup_test_db();
        add_user(&connection, "Alice", 30).expect("Failed to add user");
        let users = fetch_all_users(&connection).expect("Failed to fetch users");

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].1, "Alice");
        assert_eq!(users[0].2, 30);
    }

    #[test]
    fn test_modify_user_age() {
        let connection = setup_test_db();
        add_user(&connection, "Bob", 25).expect("Failed to add user");

        modify_user_age(&connection, 1, 35).expect("Failed to modify user age");
        let users = fetch_all_users(&connection).expect("Failed to fetch users");

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].1, "Bob");
        assert_eq!(users[0].2, 35);
    }

    #[test]
    fn test_remove_user() {
        let connection = setup_test_db();
        add_user(&connection, "Charlie", 40).expect("Failed to add user");

        let users_before = fetch_all_users(&connection).expect("Failed to fetch users");
        assert_eq!(users_before.len(), 1);

        remove_user(&connection, 1).expect("Failed to remove user");
        let users_after = fetch_all_users(&connection).expect("Failed to fetch users");

        assert_eq!(users_after.len(), 0);
    }

    #[test]
    fn test_fetch_all_users() {
        let connection = setup_test_db();
        add_user(&connection, "Dave", 20).expect("Failed to add user");
        add_user(&connection, "Eve", 22).expect("Failed to add user");

        let users = fetch_all_users(&connection).expect("Failed to fetch users");
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].1, "Dave");
        assert_eq!(users[1].1, "Eve");
    }
}
