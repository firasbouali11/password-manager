use std::io::{stdin, stdout, Write};

fn readFromCMD() -> String {
    let mut data = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut data).unwrap();
    data.trim().to_string()
}

pub fn getPasswordFromCMD() -> String {
    print!("Enter password: ");
    readFromCMD()
}

pub fn getInputFromCMDForProfile() -> (String, String, String) {
    print!("Enter profile name: ");
    let profile_name= readFromCMD();
    print!("Enter profile email: ");
    let email= readFromCMD();
    print!("Enter profile password: ");
    let password= readFromCMD();
    (profile_name, email, password)
}

pub fn getInputForSignUp() -> (String, String, String) {
    print!("Enter email: ");
    let email= readFromCMD();
    print!("Enter password: ");
    let password = readFromCMD();
    print!("Confirm password: ");
    let password_confirmation= readFromCMD();
    (email, password, password_confirmation)
}