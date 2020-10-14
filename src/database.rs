use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;

pub struct DbPool(Pool<ConnectionManager<PgConnection>>);

pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);

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

pub fn create_db_pool() -> DbPool {
    let credentials = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(credentials);

    DbPool(Pool::new(manager).expect("Failed to create database pool"))
}
