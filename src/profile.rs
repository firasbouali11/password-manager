use std::error::Error;
use rusqlite::{Connection, params};
use crate::security::{decrypt, encrypt};

#[derive(Debug)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub application_name: String,
    pub email: String,
    pub password: String
}

pub fn saveProfile(conn: &Connection, application_name: &str, user_id: i32, email: &str, password: String){
    let enc_password = encrypt(password, user_id);
    conn.execute("INSERT INTO profile(user_id, application_name, email, password) VALUES(?, ?, ?, ?)", (user_id, application_name, email, enc_password))
        .expect("Something Wrong with the query");
}

pub fn getProfilePassword(conn: &Connection, application_name: &str, user_id: i32, email: &str) -> Result<String, Box<dyn Error>> {
    let stmt = conn.prepare("SELECT password FROM profile where application_name=? and email=? and user_id=?");
    let profile_password: rusqlite::Result<String> = stmt.expect("Something Wrong with the query")
        .query_row(params![application_name, email, user_id], |row| row.get(0));
    match profile_password {
        Ok(v) => Ok(decrypt(v, user_id)),
        Err(e) => Err(Box::try_from(e).unwrap())
    }
}

pub fn getAllProfiles(conn: &Connection, user_id: i32) -> Result<Vec<Profile>, Box<dyn Error>> {
    let stmt = conn.prepare("select id, application_name, email from profile where user_id = ?");
    let profiles = stmt.expect("Query Error").query_map([&user_id], |row| {
        Ok(Profile{ id: row.get(0)?, user_id, application_name: row.get(1)?, email: row.get(2)?, password: "".to_string() })
    })?.try_fold(Vec::new(), |mut acc, row| {
        acc.push(row?);
        Ok::<Vec<Profile>, Box<dyn Error>>(acc)
    })?;
    Ok(profiles)
}