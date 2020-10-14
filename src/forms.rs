#[derive(FromForm)]
pub struct UserSignUp {
    pub username: String,
    pub password: String,
}

impl UserSignUp {
    pub fn is_valid(&self) -> bool {
        true
    }
}
