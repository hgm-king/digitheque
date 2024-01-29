use chrono::prelude::*;
use pwhash::bcrypt;
use sanitize_html::{rules::predefined::DEFAULT, sanitize_str};
use std::{io, fs};

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

// Load public certificate from file.
pub fn load_certs(filename: &str) -> io::Result<Vec<rustls::Certificate>> {
    // Open certificate file.
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    let certs = rustls_pemfile::certs(&mut reader)
        .map_err(|_| error("failed to load certificate".into()))?;
    Ok(certs.into_iter().map(rustls::Certificate).collect())
}

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

// Load private key from file.
pub fn load_private_key(filename: &str) -> io::Result<rustls::PrivateKey> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map_err(|_| error("failed to load private key".into()))?;

    Ok(rustls::PrivateKey(keys[0].clone()))
}

#[test]
fn test_encryption() {
    // Hash a password with default parameters.
    let h_new = encrypt("password");

    assert!(verify("password", &h_new));
}
