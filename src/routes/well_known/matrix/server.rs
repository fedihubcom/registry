use rocket::http::ContentType;
use rocket::response::content::Content;

#[get("/.well-known/matrix/server")]
pub fn show() -> Content<String> {
    Content(
        ContentType::JSON,
        "{ \"m.server\": \"matrix.fedihub.com\" }".to_string(),
    )
}
