use crate::controllers::*;
use rocket::*;

/// Mount all rockets routes
pub fn build(rocket: Rocket) -> Rocket {
    rocket
        .register(catchers![
            catcher::internal_error,
            catcher::not_found,
            catcher::unauthorized
        ])
        .mount(
            "/posts",
            routes![
                post::index,
                post::show,
                post::store,
                post::update,
                post::destroy
            ],
        )
        .mount(
            "/users",
            routes![
                user::index,
                user::show,
                user::store,
                user::update,
                user::destroy
            ],
        )
        .mount("/auth", routes![auth::login])
}
