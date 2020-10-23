mod donate;
mod home;
mod reports;
mod sessions;
mod team;
mod users;
mod well_known;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        donate::index,
        home::index,
        reports::index,
        sessions::new,
        sessions::create,
        sessions::delete,
        team::index,
        users::new,
        users::create,
        well_known::matrix::server::show,
    ]
}
