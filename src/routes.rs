use crate::database;
use crate::models;
use crate::forms;

use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;

#[derive(Debug, rocket::response::Responder)]
#[response(content_type = "text/html")]
enum IndexResponse {
    #[response(status = 500)]
    UnknownError(()),
}

#[derive(Debug, rocket::response::Responder)]
#[response(content_type = "text/html")]
enum UserSignUpResponse {
    #[response(status = 422)]
    InvalidForm(Template),
    #[response(status = 500)]
    UnknownError(()),
}

#[derive(Serialize)]
struct BasicTemplateContext {
    layout: &'static str,
}

#[derive(Serialize)]
struct IndexTemplateContext {
    layout: &'static str,
    users: Vec<models::User>,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, sign_up_show, sign_up]
}

#[get("/")]
fn index(db_conn: database::DbConn) -> Result<Template, IndexResponse> {
    let all_users = models::User::all(db_conn)?;

    Ok(Template::render("index", &IndexTemplateContext {
        layout: "site",
        users: all_users,
    }))
}

#[get("/sign_up")]
fn sign_up_show() -> Template {
    Template::render("sign_up", &BasicTemplateContext {
        layout: "site",
    })
}

#[post("/users", data = "<form>")]
fn sign_up(
    db_conn: database::DbConn,
    form: Form<forms::UserSignUp>,
) -> Result<Redirect, UserSignUpResponse>
{
    models::NewUser::from_form(form.0)?
        .save(db_conn)?;

    Ok(Redirect::to(uri!(index)))
}

impl From<diesel::result::Error> for IndexResponse {
    fn from(_: diesel::result::Error) -> Self {
        Self::UnknownError(())
    }
}

impl From<validator::ValidationErrors> for UserSignUpResponse {
    fn from(_validation_errors: validator::ValidationErrors) -> Self {
        Self::InvalidForm(Template::render("sign_up", &BasicTemplateContext {
            layout: "site",
        }))
    }
}

impl From<diesel::result::Error> for UserSignUpResponse {
    fn from(_: diesel::result::Error) -> Self {
        Self::UnknownError(())
    }
}
