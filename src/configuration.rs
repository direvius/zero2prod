use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DbSettings,
    pub app_port: u16,
}

#[derive(Deserialize)]
pub struct DbSettings {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

pub fn read_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(
            config::File::new("app_settings.toml", config::FileFormat::Toml)
        )
        .build()?;
    settings.try_deserialize()
}