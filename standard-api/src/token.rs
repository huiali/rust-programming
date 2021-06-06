use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Token {
    pub iat: i64,
    pub exp: i64,
    pub user_id: String,
    pub enterprise_id: String,
    pub group_id: String,
    pub menu_id: String,
    pub child_enterprise_id: String,
    pub permissions: i32,
    pub iss: String,
    pub aud: String,
}

impl Token {
    pub fn decode(token: String) -> Result<Self, ()> {
        let value = env::var("RSA_PUBLIC_KEY").expect("RSA_PRIVATE_KEY not found.");
        let ras_key = &value.replace("`", "\n");
        let encoding_key = &DecodingKey::from_rsa_pem(ras_key.as_bytes()).unwrap();

        if let Ok(token_data) = decode::<Token>(
            &token.to_string(),
            &encoding_key,
            &Validation::new(Algorithm::RS256),
        ) {
            return Ok(token_data.claims);
        }

        Err(())
    }
}
