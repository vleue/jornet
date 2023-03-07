use base64::{engine::general_purpose::STANDARD, Engine};
use biscuit_auth::{KeyPair, PrivateKey};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_host: String,
    pub application_port: u16,
    pub private_key: Option<String>,
    pub github_admin_app: OAuth,
}

#[derive(Deserialize, Debug)]
pub struct OAuth {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Settings {
    let file = std::env::var("CONFIGURATION").unwrap_or_else(|_| "configuration.dhall".to_string());
    serde_dhall::from_file(file).parse().unwrap()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

impl Settings {
    pub fn get_keypair(&self) -> KeyPair {
        self.private_key
            .as_ref()
            .and_then(|pk_string| STANDARD.decode(pk_string).ok())
            .and_then(|pk_bytes| PrivateKey::from_bytes(&pk_bytes).ok())
            .map(KeyPair::from)
            .unwrap_or_else(KeyPair::new)
    }
}
