use rand::RngCore;
use rocket::{Data, Request};
use rocket::fairing::{Fairing as RocketFairing, Info, Kind};
use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome};

const COOKIE_NAME: &str = "csrf_token";
const RAW_TOKEN_LENGTH: usize = 32;

pub struct Fairing;

pub struct Guard(Vec<u8>);

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
        if let Some(_) = valid_raw_from_request(request) { return }

        let mut raw = [0u8; RAW_TOKEN_LENGTH];
        rand::thread_rng().fill_bytes(&mut raw);

        let encoded = base64::encode(raw);

        request.cookies().add_private(Cookie::new(COOKIE_NAME, encoded));
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Guard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match valid_raw_from_request(request) {
            None => Outcome::Failure((Status::Forbidden, ())),
            Some(old_raw) => Outcome::Success(Self(old_raw)),
        }
    }
}

fn valid_raw_from_request(request: &Request) -> Option<Vec<u8>> {
    request.cookies().get_private(COOKIE_NAME)
        .and_then(|cookie| base64::decode(cookie.value()).ok())
        .and_then(|raw|
            if raw.len() >= RAW_TOKEN_LENGTH { Some(raw) } else { None }
        )
}
