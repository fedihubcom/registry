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

    let template_context = IndexTemplateContext {
        layout: "site",
        users: all_users,
    };

    Ok(Template::render("index", &template_context))
}

#[get("/sign_up")]
fn sign_up_show() -> Template {
    let template_context = BasicTemplateContext {
        layout: "site",
    };

    Template::render("sign_up", &template_context)
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

impl From<()> for IndexResponse {
    fn from(_: ()) -> Self {
        Self::UnknownError(())
    }
}

impl From<validator::ValidationErrors> for UserSignUpResponse {
    fn from(_validation_errors: validator::ValidationErrors) -> Self {
        let template_context = BasicTemplateContext {
            layout: "site",
        };

        Self::InvalidForm(Template::render("sign_up", &template_context))
    }
}

impl From<()> for UserSignUpResponse {
    fn from(_: ()) -> Self {
        Self::UnknownError(())
    }
}
