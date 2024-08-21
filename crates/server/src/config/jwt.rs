use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Debug,Clone)]
pub struct JwtConfig{
    pub secret:String,
    pub exp:u64
}
impl JwtConfig{
    pub fn encoding_key(&self)->EncodingKey{
        EncodingKey::from_secret(self.secret.as_bytes())
    }
    pub fn decoding_key(&self)->DecodingKey{
        DecodingKey::from_secret(self.secret.as_bytes())
    }
}