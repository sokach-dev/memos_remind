use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::error::Error;

fn send_email() -> Result<(), Box<dyn Error>> {
    let from = "xxxx"; // 发件邮箱
    let to = "yyyy@163.com"; // 收件邮箱

    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject("Rust Email Test")
        .body("Hello from Rust with lettre!".to_string())?;

    let smtp_server = "smtp.qq.com"; // 根据邮件服务商而定
    let smtp_username = "xxxx"; // 发件邮箱
    let smtp_password = "xxxxxxx"; // 授权码,不同邮件服务商获取方式有所不同,可搜索解决;qq邮箱可参考 https://codeantenna.com/a/PwKbc0S5dd

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

fn main() {
    if let Err(e) = send_email() {
        eprintln!("An error occurred: {}", e);
    }
}
