use crate::config;
use crate::database;
use crate::i18n::{self, I18n};
use crate::routes;

use rocket_contrib::serve::{Options as ServeOptions, StaticFiles};
use rocket_contrib::templates::Template;

pub fn rocket(config: &config::Config) -> Result<rocket::Rocket, ()> {
    let rocket_config = config.to_rocket_config()?;

    let public_path  = config.public_path()?;
    let locales_path = config.locales_path()?;

    let i18n = match I18n::new(&locales_path, &["en", "ru"]) {
        Err(_) => return Err(()),
        Ok(i18n) => i18n,
    };

    let i18n_arc = match I18n::new(&locales_path, &["en", "ru"]) {
        Err(_) => return Err(()),
        Ok(i18n) => std::sync::Arc::new(i18n),
    };

    let result = rocket::custom(rocket_config)
        .manage(i18n)
        .manage(database::create_db_pool(config))
        .attach(rocket_csrf::Fairing::new())
        .attach(Template::custom(move |engines| {
            engines.handlebars.set_strict_mode(true);

            let i18n_arc = i18n_arc.clone();

            engines.handlebars.register_helper(
                "translate",
                i18n::handlebars_helpers::translate(i18n_arc),
            );
        }))
        .mount("/", routes::routes())
        .mount("/", StaticFiles::new(public_path, ServeOptions::None));

    Ok(result)
}
