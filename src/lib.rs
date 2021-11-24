#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
extern crate jsonwebtoken;
// extern crate crypto;
extern crate pwhash;
extern crate validator;

pub mod response;
pub mod database;
pub mod models;
pub mod schema;
pub mod controllers;
pub mod routes;
pub mod guards;
pub mod fairings;