use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub database: DatabaseSettings,
    pub http_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_config() -> Result<AppConfig, config::ConfigError> {
    // Init the config reader.
    let mut cfg = config::Config::default();
    // Load the config.(yaml|toml|...) file.
    cfg.merge(config::File::with_name("config"))?;

    cfg.try_into()
}
