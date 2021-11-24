use rocket::*;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[catch(500)]
pub fn internal_error() -> JsonValue {
    json!({
        "message": "Whoops! Looks like we messed up."
    })
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "message": "Not found."
    })
}

#[catch(401)]
pub fn unauthorized() -> JsonValue {
    json!({
        "message": "Unauthorized."
    })
}
