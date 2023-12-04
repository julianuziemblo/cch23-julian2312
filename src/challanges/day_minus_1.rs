use rocket::get;
use rocket::http::Status;

#[get("/")]
pub fn index() -> &'static str {
    "Kocham Natalię"
}

#[get("/-1/error")]
pub fn error() -> Status {
    Status::new(500)
}