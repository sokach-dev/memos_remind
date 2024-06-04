use macros::config;
use validator::Validate;

#[config("REMIND_CONFIG")]
#[derive(Debug, serde::Deserialize, Validate)]
pub struct Config {
    #[validate(length(min = 1))]
    pub note_paths: Vec<String>, // 笔记的路径

    #[validate(length(min = 1))]
    pub email_from: String, // 发送邮件的邮箱
    #[validate(length(min = 1))]
    pub email_to: String, // 接收邮件的邮箱
    #[validate(length(min = 1))]
    pub smtp_server: String, // SMTP 服务器
    #[validate(length(min = 1))]
    pub smtp_username: String, // SMTP 用户名
    #[validate(length(min = 1))]
    pub smtp_password: String, // SMTP 密码
}
