use crate::utils::b64u_decode;
use crate::{Error, Result};
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| -> Config {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("Failed to load config: {}", ex);
        })
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub WEB_FOLDER: String,
    pub DB_URL: String,
    pub PWD_KEY: Vec<u8>,
    pub TOKEN_KEY: Vec<u8>,
    pub TOKEN_DURATION_SEC: u64,
}

impl Config {
    fn load_from_env() -> Result<Self> {
        Ok(Config {
            PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,
            DB_URL: get_env("SERVICE_DB_URL")?,
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse().map_err(|_| Error::ConfigInvalidEnv(name))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    b64u_decode(&get_env(name)?).map_err(|_| Error::ConfigMissingEnv(name))
}
