use crate::database::Connection;
use crate::guards::auth::Claims;
use crate::models::user::User;
use rocket::*;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

#[get("/")]
pub fn index(conn: Connection, _claims: Claims) -> JsonValue {
    let results = User::all(&conn);
    json!(&results)
}

#[get("/<_id>")]
pub fn show(_id: i64, conn: Connection, _claims: Claims) -> JsonValue {
    let result = User::find(_id, &conn);
    json!(&result)
}

#[post("/", data = "<data>")]
pub fn store(data: Json<User>, conn: Connection, _claims: Claims) -> JsonValue {
    let result = User::create(data.into_inner(), &conn);
    json!(&result)
}

#[put("/<_id>", data = "<data>")]
pub fn update(_id: i64, data: Json<User>, conn: Connection, _claims: Claims) -> JsonValue {
    let result = User::update(_id, data.into_inner(), &conn);
    json!(&result)
}

#[delete("/<_id>")]
pub fn destroy(_id: i64, conn: Connection, _claims: Claims) -> JsonValue {
    let success = User::destroy(_id, &conn);
    json!({ "success": success })
}
