
// encryption athbash cipher
pub fn encrypt(plaintext: &str) -> String {
    let mut ciphertext = String::new();

    for c in plaintext.chars() {
        if c.is_ascii_uppercase() {
            let encrypted_char = (b'Z' - (c as u8 - b'A')) as char;
            ciphertext.push(encrypted_char);
        } else {
            ciphertext.push(c);
        }
    }
    ciphertext
}

// decryption athbash cipher
pub fn decrypt(ciphertext: &str) -> String {
    let mut plaintext = String::new();

    for c in ciphertext.chars() {
        if c.is_ascii_uppercase() {
            let decrypted_char = (b'Z' - (c as u8 - b'A')) as char;
            plaintext.push(decrypted_char);
        } else {
            plaintext.push(c);
        }
    }
    plaintext
}