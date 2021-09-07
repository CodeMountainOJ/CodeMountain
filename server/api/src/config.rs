use config::Config;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref CONFIGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(config::File::with_name("config.toml")).unwrap();

        settings
    });
}

pub fn get<'a, T: serde::Deserialize<'a>>(key: &str) -> T {
    CONFIGS.read().unwrap().get::<T>(key).unwrap()
}

/// Checks config for required values
///
/// # Panics
/// Panics when required value is not found
pub fn run_config_check() {
    get::<String>("DATABASE_URL");
    get::<String>("JWT_SECRET_KEY");
    get::<String>("LISTENING_URL");
}