use crate::shared::sha3::sha3_256_of_file;
use std::fs;
use tokio_postgres::{Client};

pub async fn check_table_exists(client: &Client) -> Result<bool, tokio_postgres::Error> {
    let rows = client
        .query(
            "SELECT EXISTS (
                SELECT 1 FROM information_schema.tables
                WHERE table_schema = 'public' AND table_name = 't_migration'
            )",
            &[],
        )
        .await?;

    Ok(rows[0].get(0))
}

pub async fn create_migration_table(client: &Client) -> Result<(), tokio_postgres::Error> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS t_migration (
                id SERIAL PRIMARY KEY,
                file_name TEXT NOT NULL UNIQUE,
                checksum_sha3 TEXT NOT NULL,
                executed_at TIMESTAMP DEFAULT now()
            )",
            &[],
        )
        .await?;

    Ok(())
}

pub async fn run_migrations(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let migration_files = fs::read_dir("./migrations")?;

    let entries: Vec<_> = migration_files.collect::<Result<_, _>>()?;
    // entries.reverse();

    for entry in entries {
        let path = entry.path();

        let file_name = path
            .file_name()
            .ok_or("Invalid file name")?
            .to_string_lossy();

        println!("{}", file_name);

        if path.extension().map_or(false, |ext| ext == "sql") {
            let file_name = path
                .file_name()
                .ok_or("Invalid file name")?
                .to_string_lossy()
                .to_string();

            let file_sha3 = sha3_256_of_file(path.to_str().unwrap())?;

            let rows = client
                .query(
                    "SELECT checksum_sha3 FROM t_migration WHERE file_name = $1",
                    &[&file_name],
                )
                .await?;

            if let Some(row) = rows.get(0) {
                let db_sha3: String = row.get(0);
                if db_sha3 != file_sha3 {
                    panic!(
                        "The SHA3 checksum of the file {} does not match the one recorded in the table.",
                        file_name
                    );
                }
                println!("Already executed and verified: {}", file_name);
                continue;
            }

            let sql_content = fs::read_to_string(&path)?;
            client.batch_execute(&sql_content).await?;

            client
                .execute(
                    "INSERT INTO t_migration (file_name, checksum_sha3) VALUES ($1, $2)",
                    &[&file_name, &file_sha3],
                )
                .await?;

            println!("Excecuted: {}", file_name);
        }
    }

    Ok(())
}
