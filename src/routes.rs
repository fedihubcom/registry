mod home;
mod sessions;
mod users;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        home::index,
        sessions::new,
        users::new,
        users::create,
    ]
}
