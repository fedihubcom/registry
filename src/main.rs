#![feature(decl_macro, proc_macro_hygiene)]

#[cfg(test)] mod tests;

mod config;
mod database;
mod forms;
mod models;
mod responses;
mod routes;
mod schema;
mod states;
mod views;
mod web;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

fn main() {
    dotenv::dotenv().unwrap();
    let config = config::Config::from_env().unwrap();
    println!("Running with {:#?}", config);
    println!("Public path: {:#?}", config.public_path().unwrap());
    web::rocket(&config).unwrap().launch();
}
