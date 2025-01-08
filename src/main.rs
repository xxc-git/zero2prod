use zero2prod::startup::run;
use zero2prod::configuration;
use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() ->std::io::Result<()> {
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::sink
    );
    init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    println!("{} {}", configuration.database.require_ssl, configuration.database.port);
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
