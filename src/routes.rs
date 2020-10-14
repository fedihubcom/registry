use crate::database;
use crate::schema;
use crate::models;
use crate::forms;

use diesel::prelude::*;
use rocket::response::Redirect;
use rocket::request::Form;
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

#[post("/users", data = "<user_sign_up_form>")]
fn sign_up(
    db_conn: database::DbConn,
    user_sign_up_form: Form<forms::UserSignUp>,
) -> Redirect
{
    use schema::users;

    if !user_sign_up_form.is_valid() {
        return Redirect::to(uri!(index));
    }

    let encrypted_password = bcrypt::hash(
        user_sign_up_form.password.to_string(),
        bcrypt::DEFAULT_COST,
    ).unwrap();

    let new_user = models::NewUser {
        username: user_sign_up_form.username.as_str(),
        encrypted_password: encrypted_password.as_str(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<models::User>(&*db_conn)
        .expect("Error creating user");

    Redirect::to(uri!(index))
}
