use crate::database::DbConn;

use crate::schema::reports;

use diesel::prelude::*;

#[derive(Debug, Identifiable, Serialize, Queryable)]
pub struct Report {
    pub id: i32,
    pub datetime: std::time::SystemTime,
    pub party: String,
    pub amount: String,
    pub download: String,
}

impl Report {
    pub fn all(db_conn: DbConn) -> Result<Vec<Self>, diesel::result::Error> {
        reports::table.load::<Self>(&*db_conn)
    }
}
