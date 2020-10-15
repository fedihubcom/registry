use crate::database;
use crate::models;
use crate::forms;

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
    let all_users = models::User::all(db_conn).unwrap();

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

    new_user.save(db_conn).unwrap();

    Redirect::to(uri!(index))
}
