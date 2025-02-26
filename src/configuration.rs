use sqlx::postgres::PgConnectOptions;

use crate::domain::SubscriberEmail;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub email_client: EmailClientSettings,
}

#[derive(serde::Deserialize)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
    pub auth_token: String,
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<SubscriberEmail, String> {
        SubscriberEmail::parse(self.sender_email.clone())
    }
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.username)
            .password(&self.password)
            .host(&self.host)
            .port(self.port)
    }
    
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed finding current directory.");
    let configuration_directory = base_path.join("configuration");


    let app_environment = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "local".into());
    let environment = match app_environment.as_str() {
        "production" => "production.yaml",
        _ => "local.yaml" 
    };

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base.yaml")))
        .add_source(config::File::from(configuration_directory.join(environment)))
        .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;

    settings.try_deserialize()
}