use std::sync::OnceLock;
pub enum DebugLevel {
    INFO,
    DEBUG,
    WARNING,
    ERROR,
}
impl DebugLevel {
    pub fn as_num(&self) -> usize {
        match self {
            DebugLevel::INFO => 0,
            DebugLevel::DEBUG => 1,
            DebugLevel::WARNING => 2,
            DebugLevel::ERROR => 4,
        }
    }
}

pub struct Config {
    pub debug_level: DebugLevel,
}
static CONFIGS: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIGS.get_or_init(|| Config {
        debug_level: DebugLevel::INFO,
    })
}
