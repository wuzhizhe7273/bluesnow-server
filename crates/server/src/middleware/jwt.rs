use std::sync::OnceLock;
use std::time::Duration;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

pub static DECODE_HEADER: OnceLock<Validation> = OnceLock::new();
pub static ENCODE_HEADER: OnceLock<Header> = OnceLock::new();

#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct Claims{
    pub sub:Uuid,
    pub iat:u64,
    pub exp:u64
}

impl Claims{
    pub fn new(sub: Uuid, exp: Duration) -> Self {
        let iat = Utc::now().timestamp() as u64;
        let exp = iat + exp.as_secs();
        Self { sub, exp, iat }
    }
    pub fn decode(
        token: &str,
        key: &DecodingKey,
    ) -> std::result::Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        let decode_header = DECODE_HEADER.get_or_init(|| Validation::new(Algorithm::HS256));
        jsonwebtoken::decode::<Self>(token, key, &decode_header)
    }
    pub fn encode(
        &self,
        key: &EncodingKey,
    ) -> std::result::Result<String, jsonwebtoken::errors::Error> {
        let encode_header = ENCODE_HEADER.get_or_init(|| Header::new(Algorithm::HS256));
        jsonwebtoken::encode(&encode_header, self, key)
    }
}