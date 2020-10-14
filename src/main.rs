#![feature(decl_macro, proc_macro_hygiene)]

mod database;
mod schema;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate dotenv;
extern crate rocket_contrib;

use diesel::prelude::*;
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    parent: &'static str,
    users: Vec<String>,
}

#[derive(Debug, Queryable)]
struct User {
    pub id: i32,
    pub username: String,
}

fn main() {
    dotenv::dotenv().ok();
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(database::create_db_pool())
        .attach(Template::fairing())
        .mount("/", routes())
}

fn routes() -> Vec<rocket::Route> {
    routes![index]
}

#[get("/")]
fn index(db_conn: database::DbConn) -> Template {
    use schema::users::dsl::*;

    let all_users = users.load::<User>(&*db_conn).expect("Error loading users");
    let all_user_names = all_users.iter().map(|user| user.username.to_string()).collect();

    let template_context = TemplateContext {
        parent: "layout",
        users: all_user_names,
    };

    Template::render("index", &template_context)
}
