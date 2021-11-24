use crate::database::Connection;
use crate::models::user::User;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
}

impl Claims {
    pub fn token(&self, secret: &str) -> String {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("token")
    }

    pub fn user(&self, conn: Connection) -> Option<User> {
        User::find(self.sub, &conn)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Claims {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Claims, Self::Error> {
        if let Some(auth) = extract_auth_from_request(request, &env::var("SECRET").expect("SECRET"))
        {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

pub fn extract_auth_from_request(request: &Request, secret: &str) -> Option<Claims> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
}

pub fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with("Bearer ") {
        Some(&header[7..])
    } else {
        None
    }
}

pub fn decode_token(token: &str, secret: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| {
        eprintln!("Auth decode error: {:?}", err);
    })
    .ok()
    .map(|token_data| token_data.claims)
}
