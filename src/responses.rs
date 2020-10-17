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

#[derive(Serialize)]
struct ErrorTemplateContext {
    layout: &'static str,
    error_code: u16,
}

impl From<rocket_csrf::VerificationFailure> for CommonResponse {
    fn from(_: rocket_csrf::VerificationFailure) -> Self {
        Self::InvalidAuthenticityToken(Redirect::to("/"))
    }
}

impl From<diesel::result::Error> for CommonResponse {
    fn from(_: diesel::result::Error) -> Self {
        let template_context = ErrorTemplateContext {
            layout: "site",
            error_code: 500,
        };

        Self::UnknownError(Template::render("error", &template_context))
    }
}
