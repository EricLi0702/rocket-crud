use dotenv::dotenv;
use rocket::config::{Config, Environment, Value};
use rocket::fairing::AdHoc;
use std::collections::HashMap;
use std::env;

pub fn build() {
    dotenv().ok();
}
