use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Setting{
    pub database: DatebaseSetting,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatebaseSetting{
    pub username: String,     
    pub password: Secret<String>,     
    pub host: String,         
    pub port: u16,            
    pub database_name: String,
}

impl DatebaseSetting {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port
        ))
    }
}

pub fn get_configuration() -> Result<Setting, config::ConfigError> {
    let setting = config::Config::builder()
        .add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
        .build()?;

    setting.try_deserialize() 
}
