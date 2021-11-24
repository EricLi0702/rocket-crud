use crate::database::Connection;
use crate::guards::auth::Claims;
use crate::models::post::Post;
use crate::response::ApiResponse;
use rocket::http::Status;
use rocket::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(conn: Connection, _claims: Claims) -> ApiResponse {
    let results = Post::all(&conn);
    ApiResponse {
        data: json!(&results),
        status: Status::Ok,
    }
}

#[get("/<_id>")]
pub fn show(_id: i64, conn: Connection, _claims: Claims) -> ApiResponse {
    let result = Post::find(_id, &conn);
    match result {
        Some(data) => ApiResponse {
            data: json!(&data),
            status: Status::Ok,
        },
        None => ApiResponse {
            data: json!({"message": "Post not found."}),
            status: Status::NotFound,
        },
    }
}

#[post("/", data = "<data>")]
pub fn store(data: Json<Post>, conn: Connection, _claims: Claims) -> ApiResponse {
    let result = Post::create(data.into_inner(), &conn);
    ApiResponse {
        data: json!(&result),
        status: Status::Ok,
    }
}

#[put("/<_id>", data = "<data>")]
pub fn update(_id: i64, data: Json<Post>, conn: Connection, _claims: Claims) -> ApiResponse {
    let result = Post::update(_id, data.into_inner(), &conn);
    ApiResponse {
        data: json!(&result),
        status: Status::Ok,
    }
}

#[delete("/<_id>")]
pub fn destroy(_id: i64, conn: Connection, _claims: Claims) -> ApiResponse {
    let success = Post::destroy(_id, &conn);
    match success {
        true => ApiResponse {
            data: json!({"message": "Post deleted successfully."}),
            status: Status::Ok,
        },
        false => ApiResponse {
            data: json!({"message": "Post deleted failure."}),
            status: Status::NotFound,
        },
    }
}
