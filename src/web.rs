use crate::config;
use crate::database;
use crate::routes;

use rocket_contrib::templates::Template;

pub fn rocket(config: config::Config) -> rocket::Rocket {
    rocket::custom(config.to_rocket_config().unwrap())
        .manage(database::create_db_pool(config))
        .attach(Template::fairing())
        .mount("/", routes::routes())
}
