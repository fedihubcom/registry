#![feature(decl_macro, proc_macro_hygiene)]

mod database;
mod routes;
mod schema;
mod models;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate dotenv;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;

fn main() {
    dotenv::dotenv().ok();
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(database::create_db_pool())
        .attach(Template::fairing())
        .mount("/", routes::routes())
}
