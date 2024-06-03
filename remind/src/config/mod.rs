use macros::config;
use validator::Validate;

#[config("REMIND_CONFIG")]
#[derive(Debug, serde::Deserialize, Validate)]
pub struct Config {
    #[validate(length(min = 1))]
    pub note_paths: Vec<String>, // 笔记的路径
}
