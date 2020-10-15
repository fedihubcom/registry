const PASSWORD_MIN_LENGTH: usize = 8;
const PASSWORD_MAX_LENGTH: usize = 128;

#[derive(FromForm)]
pub struct UserSignUp {
    pub username: String,
    pub password: String,
}

impl UserSignUp {
    pub fn is_valid(&self) -> bool {
        let username_re = regex::Regex::new(r"^[a-z][a-z0-9]{3,}$").unwrap();
        let not_password_re = regex::Regex::new(r"^\s*$").unwrap();

        username_re.is_match(self.username.as_str()) &&
            !not_password_re.is_match(self.password.as_str()) &&
            self.password.len() >= PASSWORD_MIN_LENGTH &&
            self.password.len() <= PASSWORD_MAX_LENGTH
    }
}
