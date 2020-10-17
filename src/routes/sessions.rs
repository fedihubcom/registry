use crate::database;
use crate::states;
use crate::models;
use crate::forms;

use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket_csrf::CsrfToken;

#[get("/sign_in")]
pub fn new(
    csrf: CsrfToken,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, Redirect> {
    if let Some(_) = current_user.0 {
        return Err(Redirect::to(uri!(super::home::index)));
    }

    Ok(Template::render("sessions/new", &BasicTemplateContext {
        csrf_token: csrf.0,
        layout: "site",
    }))
}

#[post("/sign_in", data = "<form>")]
pub fn create(
    csrf: CsrfToken,
    db_conn: database::DbConn,
    current_user: states::MaybeCurrentUser,
    form: Form<forms::UserSignIn>,
    mut cookies: Cookies,
) -> Result<Redirect, UserSignInResponse> {
    csrf.verify(&form.authenticity_token)?;

    if let Some(_) = current_user.0 {
        return Err(UserSignInResponse::AlreadySignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    let user = models::User::by_username(db_conn, form.username.to_string())?;

    if !user.authorize(&form.password) {
        return Err(UserSignInResponse::InvalidCredentials(
            Template::render("sessions/new", &BasicTemplateContext {
                csrf_token: csrf.0,
                layout: "site",
            })
        ));
    }

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[delete("/sign_out", data = "<form>")]
pub fn delete(
    csrf: CsrfToken,
    current_user: states::MaybeCurrentUser,
    form: Form<forms::UserSignOut>,
    mut cookies: Cookies,
) -> Result<Redirect, UserSignOutResponse> {
    csrf.verify(&form.authenticity_token)?;

    if let None = current_user.0 {
        return Err(UserSignOutResponse::NoUserSignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    cookies.remove_private(Cookie::named("user_id"));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[derive(Debug, rocket::response::Responder)]
#[response(content_type = "text/html")]
pub enum UserSignInResponse {
    AlreadySignedIn(Redirect),
    #[response(status = 403)]
    InvalidAuthenticityToken(()),
    #[response(status = 422)]
    InvalidCredentials(Template),
    #[response(status = 500)]
    UnknownError(()),
}

#[derive(Debug, rocket::response::Responder)]
#[response(context_type = "text/html")]
pub enum UserSignOutResponse {
    NoUserSignedIn(Redirect),
    #[response(status = 403)]
    InvalidAuthenticityToken(()),
}

#[derive(Serialize)]
struct BasicTemplateContext {
    csrf_token: String,
    layout: &'static str,
}

impl From<diesel::result::Error> for UserSignInResponse {
    fn from(_: diesel::result::Error) -> Self {
        Self::UnknownError(())
    }
}

impl From<rocket_csrf::VerificationFailure> for UserSignInResponse {
    fn from(_: rocket_csrf::VerificationFailure) -> UserSignInResponse {
        Self::InvalidAuthenticityToken(())
    }
}

impl From<rocket_csrf::VerificationFailure> for UserSignOutResponse {
    fn from(_: rocket_csrf::VerificationFailure) -> UserSignOutResponse {
        Self::InvalidAuthenticityToken(())
    }
}
