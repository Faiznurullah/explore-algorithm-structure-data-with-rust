pub fn encrypt(plaintext: &str, key: u8) -> String {
    let mut ciphertext = String::new();

    // Affine cipher encryption 
    for c in plaintext.chars() {
        if c.is_ascii_uppercase() {
             let encrypted_char = ((key as u8 * (c as u8 - b'A')) % 26 + b'A') as char;
            ciphertext.push(encrypted_char);
        } else {
            ciphertext.push(c);
        }
    }
    ciphertext
}

pub fn decrypt(ciphertext: &str, key: u8) -> String {
    let mut plaintext = String::new();

    // Affine cipher decryption
    let mod_inv = mod_inverse(key as i32, 26).unwrap();

    for c in ciphertext.chars() {
        if c.is_ascii_uppercase() {
            let decrypted_char = ((mod_inv * ((c as u8 - b'A') as i32)) % 26 + 26) % 26 + b'A' as i32;
            plaintext.push(decrypted_char as u8 as char);
        } else {
            plaintext.push(c);
        }
    }
    plaintext
}

fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    for x in 1..m {
        if (a * x) % m == 1 {
            return Some(x);
        }
    }
    None
}