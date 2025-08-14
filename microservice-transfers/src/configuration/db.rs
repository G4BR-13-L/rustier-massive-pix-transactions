use tokio_postgres::{Client, NoTls};

pub async fn connect_to_db() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) = tokio_postgres::Config::new()
        .host("localhost")
        .port(5453)
        .user("postgres")
        .password("postgres")
        .dbname("postgres")
        .connect(NoTls)
        .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}
