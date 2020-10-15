mod home;
mod users;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        home::index,
        users::show,
        users::create,
    ]
}
