use std::path::PathBuf;

use rocket::get;

// #[get("/1/<num1>/<num2>")]
// pub fn cube_the_bits(num1: i32, num2: i32) -> String {
//     (num1 ^ num2).pow(3).to_string()
// }


/// Will ommit any non-integer path variables
#[get("/1/<paths..>")]
pub fn sled_id_system(paths: PathBuf) -> String {
    paths.iter()
        .fold(0, 
            |acc, num| 
            acc ^ num.to_string_lossy()
                .parse::<i32>()
                .unwrap_or(0)
        )
        .pow(3)
        .to_string()
}
