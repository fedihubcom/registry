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
    csrf: rocket_csrf::Guard,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, Redirect> {
    if let Some(_) = current_user.0 {
        return Err(Redirect::to(uri!(super::home::index)));
    }

    Ok(Template::render("users/new", &BasicTemplateContext {
        csrf_token: csrf.0,
        layout: "site",
    }))
}

#[post("/sign_up", data = "<form>")]
pub fn create(
    csrf: rocket_csrf::Guard,
    db_conn: database::DbConn,
    current_user: states::MaybeCurrentUser,
    form: Form<forms::UserSignUp>,
    mut cookies: Cookies,
) -> Result<Redirect, UserSignUpResponse> {
    csrf.verify(&form.authenticity_token)?;

    if let Some(_) = current_user.0 {
        return Err(UserSignUpResponse::AlreadySignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    let user = XXXXX { form: form.0, csrf_token: csrf.0 }
        .validate()?
        .save(db_conn)?;

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[derive(Debug, rocket::response::Responder)]
#[response(content_type = "text/html")]
pub enum UserSignUpResponse {
    AlreadySignedIn(Redirect),
    #[response(status = 403)]
    InvalidAuthenticityToken(()),
    #[response(status = 422)]
    InvalidForm(Template),
    #[response(status = 500)]
    UnknownError(()),
}

#[derive(Serialize)]
struct BasicTemplateContext {
    csrf_token: String,
    layout: &'static str,
}

struct XXXXX {
    form: forms::UserSignUp,
    csrf_token: String,
}

struct YYYYY {
    csrf_token: String,
}

impl XXXXX {
    fn validate(&self) -> Result<models::NewUser, YYYYY> {
        match models::NewUser::from_form(&self.form) {
            Ok(user) => Ok(user),
            Err(_) => Err(YYYYY {
                csrf_token: self.csrf_token.to_string(),
            }),
        }
    }
}

impl From<YYYYY> for UserSignUpResponse {
    fn from(yyyyy: YYYYY) -> Self {
        Self::InvalidForm(Template::render("users/new", &BasicTemplateContext {
            csrf_token: yyyyy.csrf_token,
            layout: "site",
        }))
    }
}

impl From<diesel::result::Error> for UserSignUpResponse {
    fn from(_: diesel::result::Error) -> Self {
        Self::UnknownError(())
    }
}

impl From<rocket_csrf::VerificationFailure> for UserSignUpResponse {
    fn from(_: rocket_csrf::VerificationFailure) -> Self {
        Self::InvalidAuthenticityToken(())
    }
}
