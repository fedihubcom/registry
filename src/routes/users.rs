use crate::database;
use crate::states;
use crate::views;
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

    let page_context = views::users::New {
        authenticity_token: csrf_token.0.to_string(),
        username: "".to_string(),
    };

    let context = views::Site {
        page: "users/new".to_string(),
        page_context,
        authenticity_token: csrf_token.0.to_string(),
        current_user: None,
    };

    Ok(Template::render("site", &context))
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

    let user = models::NewUser::from_form(&form)
        .or_else(|_| Err(invalid_sign_up_form(&csrf_token.0, &form.0)))?
        .save(db_conn)
        .or_else(|_| Err(invalid_sign_up_form(&csrf_token.0, &form.0)))?;

    cookies.add_private(Cookie::new("user_id", user.id.to_string()));

    Ok(Redirect::to(uri!(super::home::index)))
}

fn invalid_sign_up_form(
    authenticity_token: &String,
    form: &forms::UserSignUp,
) -> CommonResponse {
    let page_context = views::users::New {
        authenticity_token: authenticity_token.to_string(),
        username: form.username.to_string(),
    };

    let context = views::Site {
        page: "users/new".to_string(),
        page_context,
        authenticity_token: authenticity_token.to_string(),
        current_user: None,
    };

    CommonResponse::InvalidForm(Template::render("site", &context))
}
