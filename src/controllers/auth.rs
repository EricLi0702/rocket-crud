use crate::database::Connection;
use crate::models::user::User;
use crate::response::ApiResponse;
use rocket::http::Status;
use rocket::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::env;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct Credentials {
    #[validate(
        email(message = "Email is invalid."),
        length(min = 1, message = "Email is required.")
    )]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required."))]
    pub password: String,
}

#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>, conn: Connection) -> ApiResponse {
    let data = credentials.into_inner();
    match data.validate() {
        Ok(_) => match User::find_by_email_and_password(&data.email, &data.password, &conn) {
            Some(user) => ApiResponse {
                data: json!(user.to_user_auth(&env::var("SECRET").expect("SECRET"))),
                // data: json!(user),
                status: Status::Ok,
            },
            None => ApiResponse {
                data: json!({"message":"Username or password is invalid."}),
                status: Status::NotAcceptable,
            },
        },
        Err(e) => ApiResponse {
            data: json!(e),
            status: Status::UnprocessableEntity,
        },
    }
}
