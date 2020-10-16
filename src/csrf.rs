use rocket::{Data, Request};
use rocket::fairing::{Fairing as RocketFairing, Info, Kind};

const COOKIE_NAME: &str = "csrf_token";

pub struct Fairing;

impl Fairing {
    pub fn new() -> Self {
        Self {}
    }
}

impl RocketFairing for Fairing {
    fn info(&self) -> Info {
        Info {
            name: "CSRF (Cross-Site Request Forgery) protection",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        let _token: Option<String> = request.cookies()
            .get_private(COOKIE_NAME)
            .and_then(|cookie| Some(cookie.value().to_string()));
    }
}
