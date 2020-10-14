use schema::{users};

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}
