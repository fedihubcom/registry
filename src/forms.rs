use validator::{Validate, ValidationError};

#[derive(FromForm, Validate)]
pub struct UserSignUp {
    #[validate(length(min = 4, max = 128), custom = "validate_username")]
    pub username: String,
    #[validate(length(min = 8, max = 128), custom = "validate_password")]
    pub password: String,
}

fn validate_username(value: &String) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^[a-z][a-z0-9]*$").unwrap();

    if re.is_match(value) {
        Ok(())
    }
    else {
        Err(ValidationError::new("has invalid format"))
    }
}

fn validate_password(value: &String) -> Result<(), ValidationError> {
    let bad_re = regex::Regex::new(r"^\s*$").unwrap();

    if bad_re.is_match(value) {
        Err(ValidationError::new("is blank"))
    }
    else {
        Ok(())
    }
}
