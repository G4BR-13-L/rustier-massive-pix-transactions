use crate::configuration::db::connect_to_db;
use crate::product::product::Product;
use tokio_postgres::Error;
use uuid::Uuid;

pub async fn find_all() -> Result<Vec<Product>, Error> {
    let client = connect_to_db().await?;
    let rows = client
        .query(
            "SELECT uuid, name, description, price, created_at, active, available FROM t_product",
            &[],
        )
        .await?;

    let mut products = Vec::new();
    for row in rows {
        let product = Product {
            uuid: row.get(0),
            name: row.get(1),
            description: row.get(2),
            price: row.get(3),
            created_at: row.get(4),
            active: row.get(5),
            available: row.get(6),
        };
        products.push(product);
    }

    Ok(products)
}

pub async fn find_one_by_uuid(uuid: Uuid) -> Result<Product, Error> {
    let client = connect_to_db().await?;

    let row = client.query_one("SELECT uuid, name, description, price, created_at, active, available FROM t_product WHERE uuid = $1", &[&uuid]).await?;

    Ok(Product {
        uuid: row.get(0),
        name: row.get(1),
        description: row.get(2),
        price: row.get(3),
        created_at: row.get(4),
        active: row.get(5),
        available: row.get(6),
    })
}

pub async fn delete_one_by_uuid(uuid: Uuid) -> Result<(), Error> {
    let client = connect_to_db().await?;

    client
        .execute("DELETE FROM t_product WHERE uuid = $1", &[&uuid])
        .await?;

    Ok(())
}

pub async fn save(product: Product) -> Result<Product, Error> {
    let client = connect_to_db().await?;

    client
        .execute(
            "INSERT INTO t_product (uuid, name, description, price, created_at, active, available)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &product.uuid,        // Uuid
                &product.name,        // String
                &product.description, // String
                &product.price,       // i32
                &product.created_at,  // chrono::NaiveDateTime
                &product.active,      // bool
                &product.available,   // bool
            ],
        )
        .await?;

    Ok(product)
}
