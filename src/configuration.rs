use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("{}/{}", self.connection_string_without_db(), self.db_name,)
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.user, self.password, self.host, self.port,
        )
    }
}

pub fn read_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "app_settings.toml",
            config::FileFormat::Toml,
        ))
        .build()?;
    settings.try_deserialize()
}
