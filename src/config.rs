use std::sync::LazyLock;

pub struct Config {
    pub api_port: u16,
    pub cors_allow_port: u16,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    api_port: 8080,
    cors_allow_port: 3000,
});
