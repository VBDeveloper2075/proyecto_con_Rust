use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::{distributions::Alphanumeric, Rng};
use std::io;
use anyhow::{Context, Result};

// Genera una contraseña aleatoria con la longitud especificada
pub fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect();
    
    (0..length)
        .map(|_| *chars.choose(&mut rng).unwrap())
        .collect()
}

// Genera un hash de la contraseña maestra usando Argon2
pub fn hash_master_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .context("No se pudo generar el hash de la contraseña")?
        .to_string();
    Ok(password_hash)
}

// Verifica si la contraseña coincide con el hash almacenado
pub fn verify_master_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .context("Hash de contraseña inválido")?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// Genera un ID para la base de datos
pub fn generate_random_id() -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
