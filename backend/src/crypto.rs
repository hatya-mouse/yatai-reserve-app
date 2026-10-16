use rand::Rng;

/// Generates a cryptographically secure 256-bit authentication token.
pub(super) fn gen_secure_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}
