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

#[get("/sign_up")]
pub fn new(
    csrf_token: CsrfToken,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, CommonResponse> {
    if let Some(_) = current_user.0 {
        return Err(CommonResponse::AlreadySignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    Ok(Template::render("users/new", &BasicTemplateContext {
        authenticity_token: csrf_token.0,
        layout: "site",
    }))
}

#[post("/sign_up", data = "<form>")]
pub fn create(
    csrf_token: CsrfToken,
    db_conn: database::DbConn,
    current_user: states::MaybeCurrentUser,
    form: Form<forms::UserSignUp>,
    mut cookies: Cookies,
) -> Result<Redirect, CommonResponse> {
    csrf_token.verify(&form.authenticity_token)?;

    if let Some(_) = current_user.0 {
        return Err(CommonResponse::AlreadySignedIn(
            Redirect::to(uri!(super::home::index))
        ));
    }

    let user = XXXXX { form: form.0, authenticity_token: csrf_token.0 }
        .validate()?
        .save(db_conn)?;

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Redirect::to(uri!(super::home::index)))
}

#[derive(Serialize)]
struct BasicTemplateContext {
    authenticity_token: String,
    layout: &'static str,
}

struct XXXXX {
    form: forms::UserSignUp,
    authenticity_token: String,
}

struct YYYYY {
    authenticity_token: String,
}

impl XXXXX {
    fn validate(&self) -> Result<models::NewUser, YYYYY> {
        match models::NewUser::from_form(&self.form) {
            Ok(user) => Ok(user),
            Err(_) => Err(YYYYY {
                authenticity_token: self.authenticity_token.to_string(),
            }),
        }
    }
}

impl From<YYYYY> for CommonResponse {
    fn from(yyyyy: YYYYY) -> Self {
        Self::InvalidForm(Template::render("users/new", &BasicTemplateContext {
            authenticity_token: yyyyy.authenticity_token,
            layout: "site",
        }))
    }
}
