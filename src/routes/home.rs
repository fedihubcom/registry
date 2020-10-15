use crate::database;
use crate::models;

use rocket_contrib::templates::Template;

#[get("/")]
pub fn index(db_conn: database::DbConn) -> Result<Template, IndexResponse> {
    let all_users = models::User::all(db_conn)?;

    Ok(Template::render("home/index", &IndexTemplateContext {
        layout: "site",
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
    layout: &'static str,
    users: Vec<models::User>,
}

impl From<diesel::result::Error> for IndexResponse {
    fn from(_: diesel::result::Error) -> Self {
        Self::UnknownError(())
    }
}
