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

pub mod donate {
    use crate::models;

    #[derive(Serialize)]
    pub struct Index {
        pub donation_crypto_addresses: Vec<models::DonationCryptoAddress>,
    }
}

pub mod home {
    #[derive(Serialize)]
    pub struct Index {
        pub i18n_fedihub: String,
        pub i18n_federated_services_without_censorship: String,
    }
}

pub mod reports {
    use crate::models;

    #[derive(Serialize)]
    pub struct Index {
        pub reports: Vec<models::Report>,
    }
}

pub mod sessions {
    #[derive(Serialize)]
    pub struct New {
        pub authenticity_token: String,
        pub username: String,
    }
}

pub mod team {
    use crate::models;

    #[derive(Serialize)]
    pub struct Index {
        pub employees_with_contacts: Vec<(models::Employee, Vec<models::EmployeeContact>)>,
    }
}

pub mod users {
    #[derive(Serialize)]
    pub struct New {
        pub authenticity_token: String,
        pub username: String,
    }
}
