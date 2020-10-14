#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate diesel;
extern crate dotenv;
extern crate rocket_contrib;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket_contrib::templates::Template;
use std::env;
use std::ops::Deref;

struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool =
            request.guard::<State<Pool<ConnectionManager<PgConnection>>>>()?;

        match pool.get() {
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
    users: Vec<&'static str>,
}

fn main() {
    dotenv::dotenv().ok();
    rocket().launch();
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes())
}

fn routes() -> Vec<rocket::Route> {
    routes![index]
}

fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    let credentials = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(credentials);

    Pool::new(manager).expect("Failed to create database pool")
}

#[get("/")]
fn index() -> Template {
    let template_context = TemplateContext {
        parent: "layout",
        users: vec!["foo", "bar", "car"],
    };

    Template::render("index", &template_context)
}
