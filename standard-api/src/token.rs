use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use futures::Future;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::env;
use std::pin::Pin;

#[derive(Clone)]
#[derive( Debug, Serialize, Deserialize)]
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
// impl FromRequest for Token {
//     type Config = ();
//     type Error = Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Token, Error>>>>;

//     fn from_request(
//         req: &actix_web::HttpRequest,
//         _: &mut actix_web::dev::Payload,
//     ) -> <Self as actix_web::FromRequest>::Future {
//         if let Some(authen_header) = req.headers().get(AUTHORIZATION) {
//             if let Ok(authen_str) = authen_header.to_str() {
//                 if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
//                     let auth_token = authen_str[6..authen_str.len()].trim();
//                     if let Ok(token) = Token::decode(auth_token.to_string()) {
//                          println!("middleware token is {:#?}", token);
//                         return Box::pin(async move { Ok(token) });
//                     }
//                 }
//             }
//         }
//         Box::pin(async { Err(ErrorUnauthorized("unauthorized")) })
//     }
// }

impl FromRequest for Token {
    type Config = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Token, Error>>>>;

    fn from_request(
        req: &HttpRequest,
        _: &mut Payload,
    ) -> <Self as actix_web::FromRequest>::Future {
        if let Some(token_ref) = req.extensions().get::<Token>() {
            let token = token_ref.clone();
            Box::pin(async move { Ok(token) })
        } else {
            Box::pin(async { Err(ErrorUnauthorized("unauthorized")) })
        }
    }
}