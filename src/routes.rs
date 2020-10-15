use crate::database;
use crate::models;
use crate::forms;

use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    layout: &'static str,
    users: Option<Vec<models::User>>,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, sign_up_show, sign_up]
}

#[get("/")]
fn index(db_conn: database::DbConn) -> Result<Template, Redirect> {
    let all_users = models::User::all(db_conn)
        .map_err(|_| Redirect::to(uri!(index)))?
        ;

    let template_context = TemplateContext {
        layout: "site",
        users: Some(all_users),
    };

    Ok(Template::render("index", &template_context))
}

#[get("/sign_up")]
fn sign_up_show() -> Template {
    let template_context = TemplateContext {
        layout: "site",
        users: None,
    };

    Template::render("sign_up", &template_context)
}

#[post("/users", data = "<user_sign_up_form>")]
fn sign_up(
    db_conn: database::DbConn,
    user_sign_up_form: Form<forms::UserSignUp>,
) -> Result<Redirect, Redirect>
{
    models::NewUser::from_form(user_sign_up_form.0)
        .map_err(|_| Redirect::to(uri!(sign_up_show)))?
        .save(db_conn)
        .map_err(|_| Redirect::to(uri!(sign_up_show)))?
        ;

    Ok(Redirect::to(uri!(index)))
}
