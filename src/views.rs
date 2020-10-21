use crate::models;

use serde::ser::Serialize;

#[derive(Serialize)]
pub struct Site<T: Serialize> {
    pub page: String,
    pub page_context: T,
    pub authenticity_token: String,
    pub current_user: Option<models::User>,
}

#[derive(Serialize)]
pub struct Error {
    pub error_code: u16,
}

pub mod home {
    #[derive(Serialize)]
    pub struct Index {
        pub i18n_fedihub: String,
    }
}

pub mod sessions {
    #[derive(Serialize)]
    pub struct New {
        pub authenticity_token: String,
        pub username: String,
    }
}

pub mod users {
    #[derive(Serialize)]
    pub struct New {
        pub authenticity_token: String,
        pub username: String,
    }
}