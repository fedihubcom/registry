use crate::config;
use crate::database;
use crate::routes;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

pub fn rocket(config: config::Config) -> Result<rocket::Rocket, ()> {
    let rocket_config = config.to_rocket_config()?;

    let public_path = config.public_path()?;

    let result = rocket::custom(rocket_config)
        .manage(database::create_db_pool(config))
        .attach(Template::fairing())
        .mount("/", routes::routes())
        .mount("/public", StaticFiles::from(public_path));

    Ok(result)
}
