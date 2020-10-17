use crate::database;
use crate::states;
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

    Ok(Template::render("home/index", &IndexTemplateContext {
        authenticity_token: csrf_token.0,
        layout: "site",
        current_user: current_user.0,
        users: all_users,
    }))
}

#[derive(Serialize)]
struct IndexTemplateContext {
    authenticity_token: String,
    layout: &'static str,
    current_user: Option<models::User>,
    users: Vec<models::User>,
}
