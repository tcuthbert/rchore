use dotenv::dotenv;
use std::env;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    pub client_id: String,
    pub client_secret: String,
}

impl Secrets {
    pub fn new() -> Secrets {
        confy::load("rchore", Some("config")).unwrap_or_default()
    }
}

impl ::std::default::Default for Secrets {
    fn default() -> Self { 
        dotenv().ok();

        let cfg = Self { client_id: env::var("GOOGLE_CLIENT_ID").unwrap_or_default(), client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default() };
        confy::store("rchore", Some("config"), &cfg).expect("unexpected error storing config");

        let file = confy::get_configuration_file_path("rchore", Some("config")).unwrap();
        println!("The configuration file path is: {:#?}", file);

        cfg
    }
}
