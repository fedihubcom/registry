use crate::database;
use crate::schema;
use crate::models;

use diesel::prelude::*;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    parent: &'static str,
    users: Vec<String>,
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}

#[get("/")]
fn index(db_conn: database::DbConn) -> Template {
    use schema::users::dsl::*;

    let all_users = users.load::<models::User>(&*db_conn).expect("Error loading users");
    let all_user_names = all_users.iter().map(|user| user.username.to_string()).collect();

    let template_context = TemplateContext {
        parent: "layout",
        users: all_user_names,
    };

    Template::render("index", &template_context)
}
