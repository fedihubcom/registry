mod home;
mod sessions;
mod team;
mod users;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        home::index,
        sessions::new,
        sessions::create,
        sessions::delete,
        team::index,
        users::new,
        users::create,
    ]
}
