use crate::database;
use crate::models;
use crate::states;
use crate::views;

use crate::i18n::I18n;
use crate::responses::CommonResponse;

use rocket::State;
use rocket_contrib::templates::Template;
use rocket_csrf::CsrfToken;

#[get("/reports")]
pub fn index(
    _i18n: State<I18n>,
    db_conn: database::DbConn,
    csrf_token: CsrfToken,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, CommonResponse> {
    let reports = models::Report::all(db_conn)?;

    let page_context = views::reports::Index {
        reports,
    };

    let context = views::Site {
        page: "reports/index".to_string(),
        page_context,
        authenticity_token: csrf_token.authenticity_token().to_string(),
        current_user: current_user.0,
    };

    Ok(Template::render("site", &context))
}
