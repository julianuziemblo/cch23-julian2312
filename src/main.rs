use rocket::{get, routes};
use rocket::http::Status;

#[get("/")]
fn index() -> &'static str {
    "Kocham Natalię"
}

#[get("/-1/error")]
fn error() -> Status {
    Status::new(500)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![
            index, 
            error,
        ]);

    Ok(rocket.into())
}
