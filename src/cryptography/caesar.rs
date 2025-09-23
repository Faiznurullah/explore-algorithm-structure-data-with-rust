pub fn caesar_chiper(plaintext: &str, shift: u8) -> String {
 
    let mut ciphertext = String::new();

    for c in plaintext.chars() {
        if c.is_ascii_uppercase() {
            let shifted = ((c as u8 - b'A' + shift) % 26 + b'A') as char;
            ciphertext.push(shifted);
        } else {
            ciphertext.push(c);
        }
    }

    ciphertext
}