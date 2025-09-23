
// Autokey Cipher
pub fn encrypt(plaintext: &str, keyword: &str) -> String {
    let mut ciphertext = String::new();
    let mut key = keyword.to_lowercase();
    let plaintext_lower = plaintext.to_lowercase();


    for (i, c) in plaintext_lower.chars().enumerate() {
        if c.is_alphabetic() {
            let shift = key.chars().nth(i % key.len()).unwrap() as u8 - b'a';
            let encrypted_char = ((c as u8 - b'a' + shift) % 26 + b'a') as char;
            ciphertext.push(encrypted_char);
            key.push(c);
        } else { 
            ciphertext.push(c);
        }
    }

    ciphertext
}

// Decrypt function for Autokey Cipher
pub fn decrypt(ciphertext: &str, keyword: &str) -> String {
    let mut plaintext = String::new();
    let mut key = keyword.to_lowercase();

    for (i, c) in ciphertext.chars().enumerate() {
        if c.is_alphabetic() {
            let shift = key.chars().nth(i % key.len()).unwrap() as u8 - b'a';
            let decrypted_char = ((c as u8 - b'a' + 26 - shift) % 26 + b'a') as char;
            plaintext.push(decrypted_char);
            key.push(decrypted_char);
        } else { 
              plaintext.push(c);
        }
    }

    plaintext
}