use crate::controllers;
use rusqlite::Connection;

fn get_connection() -> Connection {
    match Connection::open("./test.sqlite") {
        Ok(conn) => conn,
        Err(e) => panic!("Could not establish database connection, error: {}", e),
    }
}

pub fn init_db() -> Connection {
    let conn = get_connection();
    if let Err(e) = controllers::create_todo_table(&conn) {
        panic!("Database Error: {}", e)
    }
    conn
}
