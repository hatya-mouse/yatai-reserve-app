use rand::Rng;

/// Generates a secure token for authentication using ThreadRng.
pub(super) fn gen_secure_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}
