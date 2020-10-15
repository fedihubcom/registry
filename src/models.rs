use crate::database::DbConn;

use crate::schema::users;

use diesel::prelude::*;

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
        match users::table.load::<User>(&*db_conn) {
            Err(_) => Err(()),
            Ok(users) => Ok(users),
        }
    }
}

impl<'a> NewUser<'a> {
    pub fn save(&self, db_conn: DbConn) -> Result<(), ()> {
        match diesel::insert_into(users::table).values(self).get_result::<User>(&*db_conn) {
            Err(_) => Err(()),
            Ok(_) => Ok(()),
        }
    }
}
