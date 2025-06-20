use rpassword::prompt_password;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::RngCore;
use rand::Rng;
use rand::distributions::{Alphanumeric, Distribution, Standard};
use base64::{encode, decode};

pub fn prompt_master_key() -> String {
    prompt_password("Clave maestra: ").expect("No se pudo leer la clave maestra")
}

pub fn encrypt(master_key: &str, plaintext: &str) -> Result<String, String> {
    let key = Key::from_slice(&derive_key(master_key));
    let cipher = Aes256Gcm::new(key);
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).map_err(|e| e.to_string())?;
    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);
    Ok(encode(&result))
}

pub fn decrypt(master_key: &str, b64: &str) -> Result<String, String> {
    let data = decode(b64).map_err(|e| e.to_string())?;
    if data.len() < 12 { return Err("Datos cifrados corruptos".to_string()); }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let key = Key::from_slice(&derive_key(master_key));
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())?;
    String::from_utf8(plaintext).map_err(|e| e.to_string())
}

fn derive_key(master_key: &str) -> [u8; 32] {
    // Mejor derivación que la original, pero aún simple
    // En producción, usar PBKDF2/Argon2 sería recomendable
    let mut key = [0u8; 32];
    let bytes = master_key.as_bytes();
    
    // Primera pasada: copiar bytes disponibles o rellenar con valor estático
    for i in 0..32 {
        key[i] = *bytes.get(i).unwrap_or(&(i as u8 + 1));
    }
    
    // Segunda pasada: mezclar para aumentar entropía
    for i in 0..32 {
        let j = (i + 7) % 32;
        key[i] = key[i].wrapping_add(key[j]).wrapping_mul(0x13);
    }
    
    // Tercera pasada: aplicar bytes de la clave original a lo largo de todo el array
    for (i, b) in bytes.iter().enumerate() {
        key[i % 32] ^= b;
        // Propagar el cambio
        let next = (i + 1) % 32;
        key[next] = key[next].wrapping_add(key[i % 32]);
    }
    
    key
}

pub fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+";
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    password
}