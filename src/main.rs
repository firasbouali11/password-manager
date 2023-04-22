#![allow(non_snake_case)]

use clap::Parser;

use crate::db::initDB;
use crate::profile::{getAllProfiles, saveProfile};
use crate::user::User;
use crate::cmd::*;
use crate::display::*;
use crate::security::generateKeys;

mod user;
mod profile;
mod db;
mod cmd;
mod display;
mod security;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// login or signup
    action: String,
    #[arg(short, long)]
    email: Option<String>,
    #[arg(short, long)]
    /// Show profile's password
    show: bool,
    #[arg(short, long)]
    add_profile: bool,
}

fn main() {
    let args: Args = Args::parse();
    let action = args.action.as_str();
    let email = args.email;
    let conn = initDB();
    match action {
        "login" => {
            let password = getPasswordFromCMD();
            let user_id = User::login(&conn, email.unwrap(), password);
            match user_id {
                Some(id) => {
                    let profiles = getAllProfiles(&conn, id).unwrap_or_default();
                    displayProfiles(&conn, profiles, id,args.show);
                    if args.add_profile {
                        let (profile_name, email, password) = getInputFromCMDForProfile();
                        saveProfile(&conn, profile_name.trim(), id, email.trim(), password.trim().to_string())
                    }
                }
                None => println!("Wrong User/Password Combination")
            }
        }
        "signup" => {
            let (email, password, password_confirmation) = getInputForSignUp();
            let id = User::saveUserToDB(&conn, email.trim(), password, password_confirmation);
            generateKeys(id.unwrap());
        }
        e => println!("{} is not a valid command \nuse '--help' for more information", e)

    }
}
