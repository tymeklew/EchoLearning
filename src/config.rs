use std::env::var;
#[derive(Clone)]
pub struct Config {
    pub port: String,
    pub domain: String,
    pub email: String,
}
impl Config {
    pub fn load() -> Self {
        Self {
            port: var("PORT").unwrap_or("127.0.0.1:8080".to_string()),
            domain: var("DOMAIN").unwrap_or("127.0.0.1:8080".to_string()),
            email: var("EMAIL").unwrap_or("tymek.lewandowski@gmail.com".to_string()),
        }
    }
}
