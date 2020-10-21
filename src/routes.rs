mod donate;
mod home;
mod sessions;
mod team;
mod users;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        donate::index,
        home::index,
        sessions::new,
        sessions::create,
        sessions::delete,
        team::index,
        users::new,
        users::create,
    ]
}
