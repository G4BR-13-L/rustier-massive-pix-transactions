use tokio_postgres::{Client, NoTls};
use confik::Configuration;
use serde::Deserialize;

pub async fn connect_to_db() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::Config::new()
        .host("localhost")
        .port(5451)
        .user("postgres")
        .password("postgres")
        .dbname("customers")
        .connect(NoTls)
        .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}


#[derive(Debug, Default, Configuration)]
pub struct ExampleConfig {
    pub server_addr: String,
    #[confik(from = DbConfig)]
    pub pg: deadpool_postgres::Config,
}

#[derive(Debug, serde::Deserialize)]
#[serde(transparent)]
struct DbConfig(deadpool_postgres::Config);

impl From<DbConfig> for deadpool_postgres::Config {
    fn from(value: DbConfig) -> Self {
        value.0
    }
}

impl confik::Configuration for DbConfig {
    type Builder = Option<Self>;
}