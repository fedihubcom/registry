use crate::states;
use crate::views;

use crate::i18n::I18n;

use rocket::State;
use rocket_contrib::templates::Template;
use rocket_csrf::CsrfToken;

#[get("/team")]
pub fn index(
    _i18n: State<I18n>,
    csrf_token: CsrfToken,
    current_user: states::MaybeCurrentUser,
) -> Template {
    let context = views::Site {
        page: "team/index".to_string(),
        page_context: (),
        authenticity_token: csrf_token.authenticity_token().to_string(),
        current_user: current_user.0,
    };

    Template::render("site", &context)
}
