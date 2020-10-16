use crate::database;
use crate::states;
use crate::models;
use crate::forms;

use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;

#[get("/sign_in")]
pub fn new(
    current_user: states::MaybeCurrentUser,
) -> Result<Template, Redirect> {
    if let Some(_) = current_user.0 {
        return Err(Redirect::to(uri!(super::home::index)));
    }

    Ok(Template::render("sessions/new", &BasicTemplateContext {
        layout: "site",
    }))
}

#[post("/sign_in", data = "<form>")]
pub fn create(
    db_conn: database::DbConn,
    mut cookies: Cookies,
    current_user: states::MaybeCurrentUser,
    form: Form<forms::UserSignIn>,
) -> Result<Redirect, UserSignInResponse> {
    if let Some(_) = current_user.0 {
        return Err(UserSignInResponse::AlreadySignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    let user = models::User::by_username(db_conn, form.username.to_string())?;

    if !user.authorize(&form.password) {
        return Err(UserSignInResponse::InvalidCredentials(
            Template::render("sessions/new", &BasicTemplateContext {
                layout: "site",
            })
        ));
    }

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[delete("/sign_in")]
pub fn delete(
    mut cookies: Cookies,
    current_user: states::MaybeCurrentUser,
) -> Result<Redirect, Redirect> {
    if let None = current_user.0 {
        return Err(Redirect::to(uri!(super::home::index)));
    }

    cookies.remove_private(Cookie::named("user_id"));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[derive(Debug, rocket::response::Responder)]
#[response(content_type = "text/html")]
pub enum UserSignInResponse {
    AlreadySignedIn(Redirect),
    #[response(status = 422)]
    InvalidCredentials(Template),
    #[response(status = 500)]
    UnknownError(()),
}

#[derive(Serialize)]
struct BasicTemplateContext {
    layout: &'static str,
}

impl From<diesel::result::Error> for UserSignInResponse {
    fn from(_: diesel::result::Error) -> Self {
        Self::UnknownError(())
    }
}
