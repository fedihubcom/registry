use crate::database;
use crate::states;
use crate::views;
use crate::models;

use crate::responses::CommonResponse;

use rocket_contrib::templates::Template;
use rocket_csrf::CsrfToken;

#[get("/")]
pub fn index(
    csrf_token: CsrfToken,
    db_conn: database::DbConn,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, CommonResponse> {
    let all_users = models::User::all(db_conn)?;

    let page_context = views::home::Index {
        users: all_users,
    };

    let context = views::Site {
        page: "home/index".to_string(),
        page_context,
        authenticity_token: csrf_token.0,
        current_user: current_user.0,
    };

    Ok(Template::render("site", &context))
}
