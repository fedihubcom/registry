use crate::database;
use crate::states;
use crate::models;
use crate::forms;

use crate::responses::CommonResponse;

use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket_csrf::CsrfToken;

#[get("/sign_in")]
pub fn new(
    csrf_token: CsrfToken,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, CommonResponse> {
    if let Some(_) = current_user.0 {
        return Err(CommonResponse::AlreadySignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    Ok(Template::render("sessions/new", &BasicTemplateContext {
        authenticity_token: csrf_token.0,
        layout: "site",
    }))
}

#[post("/sign_in", data = "<form>")]
pub fn create(
    csrf_token: CsrfToken,
    db_conn: database::DbConn,
    current_user: states::MaybeCurrentUser,
    form: Form<forms::UserSignIn>,
    mut cookies: Cookies,
) -> Result<Redirect, CommonResponse> {
    csrf_token.verify(&form.authenticity_token)?;

    if let Some(_) = current_user.0 {
        return Err(CommonResponse::AlreadySignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    let user = models::User::by_username(db_conn, form.username.to_string())?;

    if !user.authorize(&form.password) {
        return Err(CommonResponse::InvalidCredentials(
            Template::render("sessions/new", &BasicTemplateContext {
                authenticity_token: csrf_token.0,
                layout: "site",
            })
        ));
    }

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[delete("/sign_out", data = "<form>")]
pub fn delete(
    csrf_token: CsrfToken,
    current_user: states::MaybeCurrentUser,
    form: Form<forms::UserSignOut>,
    mut cookies: Cookies,
) -> Result<Redirect, CommonResponse> {
    csrf_token.verify(&form.authenticity_token)?;

    if let None = current_user.0 {
        return Err(CommonResponse::NotSignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    cookies.remove_private(Cookie::named("user_id"));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[derive(Serialize)]
struct BasicTemplateContext {
    authenticity_token: String,
    layout: &'static str,
}
