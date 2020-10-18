use crate::states;
use crate::views;

use crate::responses::CommonResponse;

use rocket_contrib::templates::Template;
use rocket_csrf::CsrfToken;

#[get("/")]
pub fn index(
    csrf_token: CsrfToken,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, CommonResponse> {
    let context = views::Site {
        page: "home/index".to_string(),
        page_context: (),
        authenticity_token: csrf_token.authenticity_token().to_string(),
        current_user: current_user.0,
    };

    Ok(Template::render("site", &context))
}
