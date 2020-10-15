use crate::database::DbConn;
use crate::forms;

use crate::schema::users;

use diesel::prelude::*;
use diesel::query_builder::AsQuery;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub encrypted_password: String,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub encrypted_password: String,
}

impl User {
    pub fn all(db_conn: DbConn) -> Result<Vec<Self>, diesel::result::Error> {
        let query = users::table.as_query();

        let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);

        println!("{}", debug);

        query.load::<Self>(&*db_conn)
    }
}

impl NewUser {
    pub fn from_form(form: forms::UserSignUp) -> Result<Self, ValidationErrors> {
        form.validate()?;

        let encrypted_password = bcrypt::hash(
            form.password.to_string(),
            bcrypt::DEFAULT_COST,
        ).unwrap();

        Ok(Self {
            username: form.username,
            encrypted_password: encrypted_password,
        })
    }

    pub fn save(&self, db_conn: DbConn) -> Result<(), diesel::result::Error> {
        let query = diesel::insert_into(users::table).values(self);

        let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);

        println!("{}", debug);

        query.get_result::<User>(&*db_conn)?;

        Ok(())
    }
}
