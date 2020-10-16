use rocket::{Data, Request, Response, Rocket};
use rocket::fairing::{Fairing as RocketFairing, Info, Kind};
use rocket::http::{Cookie};

const COOKIE_NAME: &str = "csrf_token";
const EXPIRE_TIME: u32 = 2_629_746; // 1 month
const REFRESH_TIME: u32 = 604_800; // 1 week

pub struct Fairing {
    secret_key: String,
}

struct Token {
    timestamp: u32,
    value: String,
}

impl Fairing {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }
}

impl Token {
    // TODO: implement this
    fn generate() -> Self {
        Self {
            timestamp: 0,
            value: "".to_string(),
        }
    }

    fn from_cookie(cookie: &Cookie) -> Self {
        Self::from_string(cookie.value().to_string())
    }

    // TODO: implement this
    fn from_string(token: String) -> Self {
        Self {
            timestamp: 0,
            value: "".to_string(),
        }
    }

    // TODO: implement this
    fn to_string(&self) -> String {
        "".to_string()
    }

    // TODO: implement this
    fn is_expired(&self) -> bool {
        true
    }

    // TODO: implement this
    fn is_refreshable(&self) -> bool {
        true
    }

    fn not_expired_or_none(self) -> Option<Self> {
        if self.is_expired() {
            None
        }
        else {
            Some(self)
        }
    }

    fn not_refreshable_or_none(self) -> Option<Self> {
        if self.is_refreshable() {
            None
        }
        else {
            Some(self)
        }
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
        let token: Option<Token> = request.cookies()
            .get_private(COOKIE_NAME)
            .and_then(|cookie| Some(Token::from_cookie(&cookie)))
            .and_then(|token| token.not_refreshable_or_none());

        if token.is_some() { return }

        let new_token = Token::generate();

        let mut new_cookie = Cookie::new(COOKIE_NAME, new_token.to_string());

        request.cookies().add_private(new_cookie);
    }
}
