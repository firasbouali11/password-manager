use sha2::{Sha512, Digest};
use std::{fs, path};
use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};
use rand;
use rand::rngs::OsRng;
use rsa::pkcs1::*;

pub fn hash(data: String) -> String{
    let mut hasher = Sha512::new();
    hasher.update(data.as_bytes());
    let enc_password = hex::encode(hasher.finalize());
    enc_password
}

pub fn encrypt(password: String, user_id: i32) -> String {
    let pub_key = RsaPublicKey::read_pkcs1_pem_file(format!("keys/{}/public_key.pem", user_id)).unwrap();
    let data = password.as_bytes();
    let enc_data: Vec<u8> = pub_key.encrypt(&mut OsRng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    hex::encode(enc_data)
}

pub fn decrypt(enc_password: String, user_id: i32) -> String {
    let priv_key = RsaPrivateKey::read_pkcs1_pem_file(format!("keys/{}/private_key.pem", user_id)).unwrap();
    let data = hex::decode(enc_password).unwrap();
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &data).expect("failed to decrypt");
    let password = String::from_utf8(dec_data.to_owned()).unwrap();
    password
}

pub fn generateKeys(user_id: i32){
    if !path::Path::new(format!("keys/{}", user_id).as_str()).is_dir() {
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut OsRng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        if !path::Path::new("keys").is_dir() {
            fs::create_dir("keys").expect("cannot create keys folder");
        }
        fs::create_dir(format!("keys/{}",user_id.to_string())).expect("cannot create user_id folder");
        pub_key.to_pkcs1_der().unwrap().write_pem_file(format!("keys/{}/public_key.pem",user_id.to_string()),"RSA PUBLIC KEY", LineEnding::LF)
            .expect("TODO: panic message");
        priv_key.to_pkcs1_der().unwrap().write_pem_file(format!("keys/{}/private_key.pem",user_id.to_string()), "RSA PRIVATE KEY", LineEnding::LF)
            .expect("TODO: panic message");
    }
}

