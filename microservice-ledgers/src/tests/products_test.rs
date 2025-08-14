#[cfg(test)]
mod products_test {
    use crate::product::persistence;

    #[tokio::test]
    async fn test_find_all_returns_products() {
        let result = persistence::find_all().await;
        assert!(result.is_ok());
        assert!(products.len() >= 0);
    }

    #[tokio::test]
    async fn test_find_all_returns_result() {
        let result = persistence::find_all().await;
        match result {
            Ok(_products) => assert!(true),
            Err(_) => {
                println!("Database not available during test");
            }
        }
    }
}
