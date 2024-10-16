use hmac::{Hmac, Mac};
use sha1::Sha1;

pub fn hmac_sha1(key: &[u8], data: &[u8]) -> [u8; 12] {
    // Create the hasher with the key. We can use expect for Hmac algorithms as they allow arbitrary key sizes.
    let mut hasher: Hmac<Sha1> =
        Mac::new_from_slice(key).expect("HMAC algoritms can take keys of any size");

    // hash the message
    hasher.update(data);

    // finalize the hash and convert to a static array
    let hmac: [u8; 12] = hasher.finalize().into_bytes()[..12].try_into().unwrap();
    hmac
}
