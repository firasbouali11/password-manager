use std::error::Error;
use rusqlite::{Connection, params};
use crate::security::hash;

#[derive(Debug, Default)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn saveUserToDB(conn: &Connection, email: &str, password: String, password_confirmation: String) -> Result<i32, Box<dyn Error>> {
        if password == password_confirmation {
            let enc_password = hash(password);
            let _ = conn.execute("INSERT INTO user(email, password) VALUES(?, ?)", (email, enc_password)).expect("Query Error");
            let id = conn.last_insert_rowid();
            Ok(id as i32)
        } else {
            Err(Box::from("zejazklej"))
        }
    }

    #[allow(dead_code)]
    pub fn getUsersFromDB(conn: &Connection) -> Result<Vec<User>, Box<dyn Error>> {
        let stmt = conn.prepare("select id, email, password from user");
        let users = stmt.expect("Error").query_map([], |row| {
            Ok(User { id: row.get(0)?, email: row.get(1)?, password: row.get(2)? })
        })?.try_fold(Vec::new(), |mut acc, row| {
            acc.push(row?);
            Ok::<Vec<User>, Box<dyn Error>>(acc)
        })?;
        Ok(users)
    }

    pub fn getUserByEmail(conn: &Connection, email: &str) -> Result<User, Box<dyn Error>> {
        let mut stmt = conn.prepare("SELECT * from user where email = ?").expect("Query Error");
        let user = stmt.query_row(params![email], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
            })
        });
        match user {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::try_from(e).unwrap())
        }
    }

    pub fn login(conn: &Connection, email: String, password: String) -> Option<i32> {
        let user_from_db = Self::getUserByEmail(&conn, email.as_str());
        let hashed_password = hash(password);
            match user_from_db{
            Ok(user) => {
                if user.password == hashed_password {
                    return Some(user.id);
                }
                return None;
            }
            Err(_) => None
        }

    }


}
