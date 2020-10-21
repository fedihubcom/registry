use crate::database;
use crate::states;
use crate::views;
use crate::models;
use crate::forms;

use crate::i18n::I18n;
use crate::responses::CommonResponse;

use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use rocket_csrf::CsrfToken;

#[get("/sign_in")]
pub fn new(
    _i18n: State<I18n>,
    csrf_token: CsrfToken,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, CommonResponse> {
    if let Some(_) = current_user.0 {
        return Err(CommonResponse::AlreadySignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    let page_context = views::sessions::New {
        authenticity_token: csrf_token.authenticity_token().to_string(),
        username: "".to_string(),
    };

    let context = views::Site {
        page: "sessions/new".to_string(),
        page_context,
        authenticity_token: csrf_token.authenticity_token().to_string(),
        current_user: None,
    };

    Ok(Template::render("site", &context))
}

#[post("/sign_in", data = "<form>")]
pub fn create(
    _i18n: State<I18n>,
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

    let user = models::User::by_username(db_conn, form.username.to_string())
        .or_else(|_| {
            Err(invalid_sign_in_credentials(
                &csrf_token.authenticity_token(),
                &form.0,
            ))
        })?;

    if !user.authorize(&form.password) {
        return Err(invalid_sign_in_credentials(
            &csrf_token.authenticity_token(),
            &form.0,
        ));
    }

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[delete("/sign_out", data = "<form>")]
pub fn delete(
    _i18n: State<I18n>,
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

fn invalid_sign_in_credentials(
    authenticity_token: &str,
    form: &forms::UserSignIn,
) -> CommonResponse {
    let page_context = views::sessions::New {
        authenticity_token: authenticity_token.to_string(),
        username: form.username.to_string(),
    };

    let context = views::Site {
        page: "sessions/new".to_string(),
        page_context,
        authenticity_token: authenticity_token.to_string(),
        current_user: None,
    };

    CommonResponse::InvalidCredentials(
        Template::render("site", &context)
    )
}
