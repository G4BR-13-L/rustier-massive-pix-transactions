use actix_web::HttpRequest;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// Service singleton para JWT
#[derive(Clone)]
pub struct JwtService {
    secret: String,
}

impl JwtService {
    pub fn new() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "uma_chave_super_secreta_que_ninguem_sabe".to_string());
        
        Self { secret }
    }
    
    // Método principal: extrair UUID do token
    pub fn extract_customer_uuid(&self, token: &str) -> Result<Uuid, JwtError> {
        let claims = self.validate_token(token)?;
        let uuid = Uuid::parse_str(&claims.sub)
            .map_err(|_| JwtError::InvalidCustomerId)?;
        Ok(uuid)
    }
    
    // Método para validar token (caso precise)
    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtError> {
        let key = DecodingKey::from_secret(self.secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);
        
        let token_data = decode::<Claims>(token, &key, &validation)
            .map_err(JwtError::ValidationError)?;
        
        Ok(token_data.claims)
    }
    
    // Método convenience para extrair diretamente do header Authorization
    pub fn extract_uuid_from_header(&self, auth_header: Option<&str>) -> Result<Uuid, JwtError> {
        let header_value = auth_header.ok_or(JwtError::MissingToken)?;
        
        if !header_value.starts_with("Bearer ") {
            return Err(JwtError::InvalidFormat);
        }
        
        let token = &header_value[7..];
        self.extract_customer_uuid(token)
    }
}

// Se quiser ainda mais simples, pode criar funções helper
pub fn extract_customer_uuid_from_request(
    req: &HttpRequest,
    jwt_service: &JwtService
) -> Result<Uuid, JwtError> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());
    
    jwt_service.extract_uuid_from_header(auth_header)
}


// Erros simples
#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("Token não fornecido")]
    MissingToken,
    
    #[error("Formato de token inválido")]
    InvalidFormat,
    
    #[error("Customer ID inválido no token")]
    InvalidCustomerId,
    
    #[error("Erro na validação do token: {0}")]
    ValidationError(#[from] jsonwebtoken::errors::Error),
}