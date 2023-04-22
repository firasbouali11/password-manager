use crate::profile::{getProfilePassword, Profile};
use rusqlite::Connection;
use cli_table::{Style, Table};

pub fn displayProfiles(conn: &Connection, profiles: Vec<Profile>, user_id: i32, with_passwords: bool){
    let tab = profiles.into_iter().map(|profile| {
        let password = if with_passwords {
            getProfilePassword(&conn, profile.application_name.as_str(), user_id, profile.email.as_str()).unwrap()
        } else {
            "*****".to_string()
        };
        return vec!(profile.id.to_string(), profile.application_name.to_owned(), profile.email.to_owned(), password);
    }).collect::<Vec<Vec<String>>>();
    let display = tab.table().title(vec!["id", "profile", "email", "password"]).bold(true).table().display().unwrap();
    println!("{}", display);
}