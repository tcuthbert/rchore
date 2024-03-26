use anyhow::Result;
use dotenv::dotenv;
use std::env;

use serde::{Deserialize, Serialize};

const APP_NAME: &str = "rchore";
const CONF_NAME: &str = "config";

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    pub client_id: String,
    pub client_secret: String,
}

impl Secrets {
    pub fn new() -> Secrets {
        confy::load(APP_NAME, Some(CONF_NAME)).unwrap_or_default()
    }
}

impl ::std::default::Default for Secrets {
    fn default() -> Self {
        dotenv().ok();

        let cfg = Self {
            client_id: env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
            client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
        };
        confy::store(APP_NAME, Some(CONF_NAME), &cfg).expect("unexpected error storing config");

        cfg
    }
}

pub fn config_path() -> Result<String> {
    let pbuff = confy::get_configuration_file_path("rchore", Some(CONF_NAME))?;
    Ok(pbuff.to_string_lossy().into())
}
