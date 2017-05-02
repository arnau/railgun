#![feature(plugin)]
#![plugin(rocket_codegen)]

#![recursion_limit = "1024"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

mod db;
mod errors;
mod handlers;
mod models;
mod schema;

fn main() {
    dotenv::dotenv().ok();
    
    rocket::ignite()
        .manage(db::establish_connection_pool())
        .mount("/", routes![handlers::pull_requests::index])
        .launch();
}