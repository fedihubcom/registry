use crate::database;
use crate::models;
use crate::forms;

use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use validator::Validate;

#[derive(Serialize)]
struct TemplateContext {
    parent: &'static str,
    users: Option<Vec<models::User>>,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, sign_up_show, sign_up]
}

#[get("/")]
fn index(db_conn: database::DbConn) -> Template {
    let all_users = models::User::all(db_conn).unwrap();

    let template_context = TemplateContext {
        parent: "layout",
        users: Some(all_users),
    };

    Template::render("index", &template_context)
}

#[get("/sign_up")]
fn sign_up_show(db_conn: database::DbConn) -> Template {
    let template_context = TemplateContext {
        parent: "layout",
        users: None,
    };

    Template::render("sign_up", &template_context)
}

#[post("/users", data = "<user_sign_up_form>")]
fn sign_up(
    db_conn: database::DbConn,
    user_sign_up_form: Form<forms::UserSignUp>,
) -> Redirect
{
    if let Err(_) = user_sign_up_form.validate() {
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
