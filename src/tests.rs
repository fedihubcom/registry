#[cfg(test)]
mod test {
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
