mod home;
mod users;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        home::index,
        users::sign_up_show,
        users::sign_up,
    ]
}
