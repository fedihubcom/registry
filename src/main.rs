#![feature(decl_macro, proc_macro_hygiene)]

mod schema;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate dotenv;
extern crate rocket_contrib;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket_contrib::templates::Template;
use std::env;
use std::ops::Deref;

struct DbPool(Pool<ConnectionManager<PgConnection>>);

struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool =
            request.guard::<State<DbPool>>()?;

        match pool.0.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
        .manage(create_db_pool())
        .attach(Template::fairing())
        .mount("/", routes())
}

fn create_db_pool() -> DbPool {
    let credentials = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(credentials);

    DbPool(Pool::new(manager).expect("Failed to create database pool"))
}

fn routes() -> Vec<rocket::Route> {
    routes![index]
}

#[get("/")]
fn index(db_conn: DbConn) -> Template {
    use schema::users::dsl::*;

    let all_users = users.load::<User>(&*db_conn).expect("Error loading users");
    let all_user_names = all_users.iter().map(|user| user.username.to_string()).collect();

    let template_context = TemplateContext {
        parent: "layout",
        users: all_user_names,
    };

    Template::render("index", &template_context)
}
