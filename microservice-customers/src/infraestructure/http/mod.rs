// Expõe o módulo de rotas da API
pub mod routes;

// Submódulo handlers separados por domínio
pub mod handlers {
    pub mod customer_handler;
    pub mod account_handler;
    pub mod pix_handler;
}
