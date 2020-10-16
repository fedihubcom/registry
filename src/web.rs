use crate::csrf;
use crate::config;
use crate::database;
use crate::routes;

use rocket_contrib::serve::{Options as ServeOptions, StaticFiles};
use rocket_contrib::templates::Template;

pub fn rocket(config: &config::Config) -> Result<rocket::Rocket, ()> {
    let rocket_config = config.to_rocket_config()?;

    let public_path = config.public_path()?;

    let result = rocket::custom(rocket_config)
        .manage(database::create_db_pool(config))
        .attach(csrf::Fairing::new())
        .attach(Template::fairing())
        .mount("/", routes::routes())
        .mount("/", StaticFiles::new(public_path, ServeOptions::None));

    Ok(result)
}
