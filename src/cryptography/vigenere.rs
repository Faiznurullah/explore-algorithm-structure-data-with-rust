
pub fn encrypt(plaintext: &str, key: &str) -> String {
     // Vigenère cipher encryption
    let mut ciphertext = String::new();
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();  

    for (i, c) in plaintext.chars().enumerate() {
        if c.is_ascii_uppercase() {
            let k = key_bytes[i % key_len].to_ascii_uppercase() - b'A';
            let encrypted_char = ((c as u8 - b'A' + k) % 26 + b'A') as char;
            ciphertext.push(encrypted_char);
        } else {
            ciphertext.push(c);
        }
    }
    ciphertext
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    // Vigenère cipher decryption
    let mut plaintext = String::new();
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    for (i, c) in ciphertext.chars().enumerate() {
        if c.is_ascii_uppercase() {
            let k = key_bytes[i % key_len].to_ascii_uppercase() - b'A';
            let decrypted_char = ((c as u8 - b'A' + 26 - k) % 26 + b'A') as char;
            plaintext.push(decrypted_char);
        } else {
            plaintext.push(c);
        }
    }
    plaintext
}