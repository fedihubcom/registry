use crate::database::DbConn;

use crate::schema::employees;

use diesel::prelude::*;

#[derive(Debug, Serialize, Queryable)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub role: String,
}

impl Employee {
    pub fn all(db_conn: DbConn) -> Result<Vec<Self>, diesel::result::Error> {
        employees::table.load::<Self>(&*db_conn)
    }
}
