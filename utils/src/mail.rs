use anyhow::Result;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

pub fn send_email(
    from: &str,        // 发件邮箱
    to: &str,          // 收件邮箱
    smtp_server: &str, // 根据邮件服务商而定
    smtp_username: &str,
    smtp_password: &str,
    subject: &str,
    body: String,
) -> Result<()> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body)?;

    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    let mailer = SmtpTransport::relay(smtp_server)?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => eprintln!("Could not send the email: {:?}", e),
    }

    Ok(())
}
