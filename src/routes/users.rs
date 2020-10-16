use crate::database;
use crate::states;
use crate::models;
use crate::forms;

use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;

#[get("/sign_up")]
pub fn new(
    current_user: states::MaybeCurrentUser,
) -> Result<Template, Redirect> {
    if let Some(_) = current_user.0 {
        return Err(Redirect::to(uri!(super::home::index)));
    }

    Ok(Template::render("users/new", &BasicTemplateContext {
        layout: "site",
    }))
}

#[post("/sign_up", data = "<form>")]
pub fn create(
    db_conn: database::DbConn,
    current_user: states::MaybeCurrentUser,
    form: Form<forms::UserSignUp>,
    mut cookies: Cookies,
) -> Result<Redirect, UserSignUpResponse> {
    if let Some(_) = current_user.0 {
        return Err(UserSignUpResponse::AlreadySignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    let user = models::NewUser::from_form(form.0)?
        .save(db_conn)?;

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[derive(Debug, rocket::response::Responder)]
#[response(content_type = "text/html")]
pub enum UserSignUpResponse {
    AlreadySignedIn(Redirect),
    #[response(status = 422)]
    InvalidForm(Template),
    #[response(status = 500)]
    UnknownError(()),
}

#[derive(Serialize)]
struct BasicTemplateContext {
    layout: &'static str,
}

impl From<validator::ValidationErrors> for UserSignUpResponse {
    fn from(_validation_errors: validator::ValidationErrors) -> Self {
        Self::InvalidForm(Template::render("users/new", &BasicTemplateContext {
            layout: "site",
        }))
    }
}

impl From<diesel::result::Error> for UserSignUpResponse {
    fn from(_: diesel::result::Error) -> Self {
        Self::UnknownError(())
    }
}
