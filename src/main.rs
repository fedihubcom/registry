#![feature(decl_macro, proc_macro_hygiene)]

pub mod config;
mod database;
mod routes;
mod schema;
mod models;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate rocket_contrib;

use rocket_contrib::templates::Template;

fn main() {
    let config = config::Config::default().unwrap();

    rocket(config).launch();
}

fn rocket(config: config::Config) -> rocket::Rocket {
    rocket::custom(config.to_rocket_config_builder().finalize().unwrap())
        .manage(database::create_db_pool(config.database_url))
        .attach(Template::fairing())
        .mount("/", routes::routes())
}
