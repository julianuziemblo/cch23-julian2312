mod challanges;

use rocket::routes;
use challanges::{
    c_minus_one,
    c_1
};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![
            c_minus_one::index, 
            c_minus_one::error,
            //c_1::cube_the_bits,
            c_1::sled_id_system,
        ]);

    Ok(rocket.into())
}
