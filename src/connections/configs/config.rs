use std::sync::OnceLock;

pub struct Config {
    pub port: i32,
}
static CONFIGS: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIGS.get_or_init(|| Config { port: 49221 })
}
