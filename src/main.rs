mod challanges;

use rocket::routes;
use challanges::c_minus_one;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![
            c_minus_one::index, 
            c_minus_one::error,
        ]);

    Ok(rocket.into())
}
