#![feature(proc_macro_hygiene, decl_macro)]
use dotenv::dotenv;
use app::database;
use app::routes;

fn main() {
    dotenv().ok();
    let rocket = rocket::ignite().manage(database::connect());
    routes::build(rocket).launch();
}
