use crate::database::DbConn;

use crate::schema::users;

use diesel::prelude::*;
use diesel::query_builder::AsQuery;

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub encrypted_password: String,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub encrypted_password: &'a str,
}

impl User {
    pub fn all(db_conn: DbConn) -> Result<Vec<User>, ()> {
        let query = users::table.as_query();

        let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);

        println!("{}", debug);

        let result = query.load::<User>(&*db_conn);

        match result {
            Err(_) => Err(()),
            Ok(users) => Ok(users),
        }
    }
}

impl<'a> NewUser<'a> {
    pub fn save(&self, db_conn: DbConn) -> Result<(), ()> {
        let query = diesel::insert_into(users::table).values(self);

        let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);

        println!("{}", debug);

        let result = query.get_result::<User>(&*db_conn);

        match result {
            Err(_) => Err(()),
            Ok(_) => Ok(()),
        }
    }
}
