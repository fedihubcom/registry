use crate::views;

use rocket::response::Redirect;
use rocket_contrib::templates::Template;

#[derive(Debug, rocket::response::Responder)]
#[response(content_type = "text/html")]
pub enum CommonResponse {
    AlreadySignedIn(Redirect),
    InvalidAuthenticityToken(Redirect),
    NotSignedIn(Redirect),
    #[response(status = 403)]
    InvalidCredentials(Template),
    #[response(status = 422)]
    InvalidForm(Template),
    #[response(status = 500)]
    UnknownError(Template),
}

impl From<rocket_csrf::VerificationFailure> for CommonResponse {
    fn from(_: rocket_csrf::VerificationFailure) -> Self {
        Self::InvalidAuthenticityToken(Redirect::to("/"))
    }
}

impl From<diesel::result::Error> for CommonResponse {
    fn from(_: diesel::result::Error) -> Self {
        let page_context = views::Error {
            error_code: 500,
        };

        let context = views::Site {
            page: "error".to_string(),
            page_context,
            authenticity_token: "".to_string(), // TODO
            current_user: None, // TODO
        };

        Self::UnknownError(Template::render("site", &context))
    }
}
