#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub encrypted_password: String,
}
