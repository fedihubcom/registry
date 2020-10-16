use rand::RngCore;
use rocket::{Data, Request};
use rocket::fairing::{Fairing as RocketFairing, Info, Kind};
use rocket::http::Cookie;

const COOKIE_NAME: &str = "csrf_token";
const RAW_TOKEN_LENGTH: usize = 32;

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
        if request.cookies().get_private(COOKIE_NAME).is_some() { return };

        let mut raw = [0u8; RAW_TOKEN_LENGTH];
        rand::thread_rng().fill_bytes(&mut raw);

        let encoded = base64::encode(raw);

        request.cookies().add_private(Cookie::new(COOKIE_NAME, encoded));
    }
}
