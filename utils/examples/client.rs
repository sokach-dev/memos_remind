use std::env;

use utils::mail::send_email;

fn main() {
    let from = env::var("EMAIL_FROM").unwrap();
    let to = env::var("EMAIL_TO").unwrap();
    let smtp_server = env::var("SMTP_SERVER").unwrap();
    let smtp_username = env::var("SMTP_USERNAME").unwrap();
    let smtp_password = env::var("SMTP_PASSWORD").unwrap();

    send_email(
        from.as_str(),
        to.as_str(),
        smtp_server.as_str(),
        smtp_username.as_str(),
        smtp_password.as_str(),
        "rust email test",
        "Hello rust email!".to_string(),
    )
    .unwrap();
}
