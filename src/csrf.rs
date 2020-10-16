use rand::RngCore;
use rocket::{Data, Request};
use rocket::fairing::{Fairing as RocketFairing, Info, Kind};
use rocket::http::{Cookie, Method, Status};
use rocket::request::{FromRequest, Outcome};

const COOKIE_NAME: &str = "csrf_token";
const PARAM_NAME: &str = "authenticity_token";
const HEADER_NAME: &str = "X-CSRF-Token";
const _PARAM_META_NAME: &str = "csrf-param";
const _TOKEN_META_NAME: &str = "csrf-token";
const RAW_TOKEN_LENGTH: usize = 32;

pub struct Fairing;

pub struct Guard;

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
        if let Some(_) = request.valid_csrf_token_from_session() { return }

        let mut raw = [0u8; RAW_TOKEN_LENGTH];
        rand::thread_rng().fill_bytes(&mut raw);

        let encoded = base64::encode(raw);

        request.cookies().add_private(Cookie::new(COOKIE_NAME, encoded));
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Guard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        if request.is_verified_against_csrf() {
            Outcome::Success(Self {})
        }
        else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

trait RequestCsrf {
    fn valid_csrf_token_from_session(&self) -> Option<Vec<u8>> {
        self.csrf_token_from_session()
            .and_then(|raw|
                if raw.len() >= RAW_TOKEN_LENGTH { Some(raw) } else { None }
            )
    }

    fn csrf_token_from_session(&self) -> Option<Vec<u8>>;

    fn csrf_token_from_header(&self) -> Option<String>;

    fn csrf_token_from_form(&self) -> Option<String>;

    fn is_verified_against_csrf(&self) -> bool;

    fn is_authenticity_token_valid(&self, token: String) -> bool {
        match self.valid_csrf_token_from_session() {
            None => false,
            Some(session_token) => false,
        }
    }
}

impl RequestCsrf for Request<'_> {
    fn csrf_token_from_session(&self) -> Option<Vec<u8>> {
        self.cookies().get_private(COOKIE_NAME)
            .and_then(|cookie| base64::decode(cookie.value()).ok())
    }

    fn csrf_token_from_header(&self) -> Option<String> {
        self.headers().get_one(HEADER_NAME).and_then(|s| Some(s.to_string()))
    }

    fn csrf_token_from_form(&self) -> Option<String> {
        self.get_query_value(PARAM_NAME).and_then(|s| s.ok())
    }

    fn is_verified_against_csrf(&self) -> bool {
        self.method() == Method::Get ||
            self.method() == Method::Head ||
            self.csrf_token_from_header().and_then(
                |token| Some(self.is_authenticity_token_valid(token))
            ).unwrap_or(false) ||
            self.csrf_token_from_form().and_then(
                |token| Some(self.is_authenticity_token_valid(token))
            ).unwrap_or(false)
    }
}
