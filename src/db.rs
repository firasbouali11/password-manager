use rusqlite::Connection;

pub fn initDB() -> Connection {
    let conn = Connection::open("db.db").unwrap();

    conn.execute("CREATE TABLE if not exists user(id INTEGER PRIMARY KEY,email TEXT NOT NULL,password TEXT NOT NULL)", ())
        .expect("TODO: panic message");

    conn.execute("CREATE TABLE if not exists profile(id INTEGER PRIMARY KEY,user_id INTEGER, application_name TEXT,email TEXT NOT NULL,password TEXT NOT NULL)", ())
        .expect("TODO: panic message");

    conn
}