use crate::csrf;
use crate::database;
use crate::states;
use crate::models;

use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(
    csrf: csrf::Guard,
    db_conn: database::DbConn,
    current_user: states::MaybeCurrentUser,
) -> Result<Template, IndexResponse> {
    let all_users = models::User::all(db_conn)?;

    Ok(Template::render("home/index", &IndexTemplateContext {
        csrf_token: csrf.0,
        layout: "site",
        current_user: current_user.0,
        users: all_users,
    }))
}

#[derive(Debug, rocket::response::Responder)]
#[response(content_type = "text/html")]
pub enum IndexResponse {
    #[response(status = 500)]
    UnknownError(()),
}

#[derive(Serialize)]
struct IndexTemplateContext {
    csrf_token: String,
    layout: &'static str,
    current_user: Option<models::User>,
    users: Vec<models::User>,
}

impl From<diesel::result::Error> for IndexResponse {
    fn from(_: diesel::result::Error) -> Self {
        Self::UnknownError(())
    }
}
