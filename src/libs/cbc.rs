use libaes::Cipher;

pub fn encrypt(key: &[u8; 16], plaintext: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Cipher::new_128(key);
    let encrypted = cipher.cbc_encrypt(iv, plaintext);
    encrypted
}

pub fn decrypt(key: &[u8; 16], ciphertext: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Cipher::new_128(key);
    let decrypted = cipher.cbc_decrypt(iv, ciphertext);
    decrypted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = b"1234567890123456";
        let plaintext = b"Hello, world!";
        let iv = b"0123456789abcdef";

        let ciphertext = encrypt(key, plaintext, iv);
        let decrypted = decrypt(key, &ciphertext, iv);

        assert_eq!(plaintext, &decrypted[..]);
    }
}
