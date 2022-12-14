use super::defaults;
use figment::{
    providers::{Format, Json},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub discord: DiscordConfiguration,
    pub database: DatabaseConfiguration,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordConfiguration {
    pub app_id: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfiguration {
    pub host: String,
    pub port: Option<u32>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
}

pub fn load_config(path: Option<&str>) -> eyre::Result<Configuration> {
    Figment::new()
        .merge(Json::file(
            path.unwrap_or(defaults::DEFAULT_CONFIG_FILE_PATH),
        ))
        .extract()
        .map_err(|err| err.into())
}
