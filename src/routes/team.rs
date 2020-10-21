use crate::database;
use crate::models;
use crate::states;
use crate::views;

use crate::i18n::I18n;
use crate::responses::CommonResponse;

use rocket::State;
use rocket_contrib::templates::Template;
use rocket_csrf::CsrfToken;

#[get("/team")]
pub fn index(
    _i18n: State<I18n>,
    db_conn: database::DbConn,
    csrf_token: CsrfToken,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, CommonResponse> {
    let employees_with_contacts = models::Employee::all_with_contacts(db_conn)?;

    let page_context = views::team::Index {
        employees_with_contacts,
    };

    let context = views::Site {
        page: "team/index".to_string(),
        page_context,
        authenticity_token: csrf_token.authenticity_token().to_string(),
        current_user: current_user.0,
    };

    Ok(Template::render("site", &context))
}
