use chrono::prelude::*;
use pwhash::bcrypt;
use sanitize_html::{rules::predefined::DEFAULT, sanitize_str};

pub fn now() -> chrono::naive::NaiveDateTime {
    Utc::now().naive_local()
}

pub fn encrypt(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}

pub fn verify(password: &str, hashed: &str) -> bool {
    bcrypt::verify(password, hashed)
}

pub fn sanitize_html(input: &str) -> String {
    sanitize_str(&DEFAULT, input).unwrap()
}

#[test]
fn test_encryption() {
    // Hash a password with default parameters.
    let h_new = encrypt("password");

    assert!(verify("password", &h_new));
}
