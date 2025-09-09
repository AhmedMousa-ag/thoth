use std::sync::OnceLock;

pub struct Config {
    pub sleep_time_min: u64,
    pub quorum: usize,
}
static CONFIGS: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIGS.get_or_init(|| {
        Config {
            sleep_time_min: 24 * 60, //Every 24 hours.
            quorum: 2,               //Minimum nodes to perform sync.
        }
    })
}
