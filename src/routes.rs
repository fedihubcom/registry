use crate::database;
use crate::schema;
use crate::models;

use diesel::prelude::*;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    parent: &'static str,
    users: Vec<models::User>,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, sign_up]
}

#[get("/")]
fn index(db_conn: database::DbConn) -> Template {
    use schema::users::dsl::*;

    let all_users = users.load::<models::User>(&*db_conn).expect("Error loading users");

    let template_context = TemplateContext {
        parent: "layout",
        users: all_users,
    };

    Template::render("index", &template_context)
}

#[post("/users/<username>/<password>")]
fn sign_up(db_conn: database::DbConn, username: String, password: String) -> Redirect {
    use schema::users;

    let encrypted_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

    let new_user = models::NewUser {
        username: username.as_str(),
        encrypted_password: encrypted_password.as_str(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<models::User>(&*db_conn)
        .expect("Error creating user");

    Redirect::to(uri!(index))
}
