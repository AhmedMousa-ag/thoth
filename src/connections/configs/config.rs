use std::sync::OnceLock;

pub struct Config {
    pub port: i32,
    pub max_msg_size: usize,
}
static CONFIGS: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIGS.get_or_init(|| Config {
        port: 49221,
        max_msg_size: 1_024_000,
    }) // 1 Gb
}
