#![feature(decl_macro, proc_macro_hygiene)]

#[cfg(test)] mod tests;

pub mod config;
mod web;
mod database;
mod routes;
mod schema;
mod models;
mod forms;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate rocket_contrib;

fn main() {
    dotenv::dotenv().unwrap();
    let config = config::Config::from_env().unwrap();
    web::rocket(config).launch();
}
