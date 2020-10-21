use crate::database::DbConn;

use crate::schema::employees;

use diesel::prelude::*;
use diesel::query_builder::AsQuery;

#[derive(Debug, Serialize, Queryable)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub role: String,
}

impl Employee {
    pub fn all(db_conn: DbConn) -> Result<Vec<Self>, diesel::result::Error> {
        let query = employees::table.as_query();

        let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);

        println!("{}", debug);

        query.load::<Self>(&*db_conn)
    }
}
