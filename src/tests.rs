#[cfg(test)]
mod requests {
    use crate::config;
    use crate::web;

    use rocket::http::Status;
    use rocket::local::Client;

    fn client() -> Client {
        let config = config::Config::default().unwrap();
        let rocket = web::rocket(config);
        Client::new(rocket).unwrap()
    }

    #[test]
    fn index() {
        let client = client();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}

#[cfg(test)]
mod forms {
    use crate::forms;

    use validator::Validate;

    #[test]
    fn user_sign_up() {
        let form = forms::UserSignUp {
            username: "kotovalexarian".to_string(),
            password: "q1w2e3r4t5y6".to_string(),
        };

        assert!(matches!(form.validate(), Ok(_)));
    }

    #[test]
    fn user_sign_up_with_empty_username() {
        let form = forms::UserSignUp {
            username: "".to_string(),
            password: "q1w2e3r4t5y6".to_string(),
        };

        assert!(matches!(form.validate(), Err(_)));
    }

    #[test]
    fn user_sign_up_with_blank_username() {
        let form = forms::UserSignUp {
            username: " ".to_string(),
            password: "q1w2e3r4t5y6".to_string(),
        };

        assert!(matches!(form.validate(), Err(_)));
    }

    #[test]
    fn user_sign_up_with_too_short_username() {
        let form = forms::UserSignUp {
            username: "foo".to_string(),
            password: "q1w2e3r4t5y6".to_string(),
        };

        assert!(matches!(form.validate(), Err(_)));
    }

    #[test]
    fn user_sign_up_with_empty_password() {
        let form = forms::UserSignUp {
            username: "kotovalexarian".to_string(),
            password: "".to_string(),
        };

        assert!(matches!(form.validate(), Err(_)));
    }

    #[test]
    fn user_sign_up_with_blank_password() {
        let form = forms::UserSignUp {
            username: "kotovalexarian".to_string(),
            password: " ".to_string(),
        };

        assert!(matches!(form.validate(), Err(_)));
    }

    #[test]
    fn user_sign_up_with_too_short_password() {
        let form = forms::UserSignUp {
            username: "kotovalexarian".to_string(),
            password: "1234567".to_string(),
        };

        assert!(matches!(form.validate(), Err(_)));
    }
}
