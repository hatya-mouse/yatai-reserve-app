use rand::Rng;
use resend_rs::{Resend, types::CreateEmailBaseOptions};

/// Generates a cryptographically secure 256-bit authentication token.
pub(super) fn gen_secure_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Sends an information email to the specified email address.
pub(super) async fn send_email(
    resend_key: String,
    from: String,
    to: String,
    subject: String,
    html_content: String,
) -> Result<(), resend_rs::Error> {
    let resend = Resend::new(&resend_key);
    let email = CreateEmailBaseOptions::new(from, [to], subject).with_html(&html_content);
    resend.emails.send(email).await.map(|_| ())
}
