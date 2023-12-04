mod challanges;

use rocket::routes;
use challanges::{
    day_minus_1,
    day_1, day_4
};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![
            day_minus_1::index, 
            day_minus_1::error,
            // * Day 1
            //day_1::cube_the_bits,
            day_1::sled_id_system,
            // * Day 2
            day_4::strength,
            day_4::contest

        ]);

    Ok(rocket.into())
}
