use std::path::PathBuf;

use rocket::get;

// #[get("/1/<num1>/<num2>")]
// pub fn cube_the_bits(num1: i32, num2: i32) -> String {
//     (num1 ^ num2).pow(3).to_string()
// }


#[get("/1/<paths..>")]
pub fn sled_id_system(paths: PathBuf) -> String {
    paths.iter()
        .map(|path| 
            path.to_string_lossy()
                .parse::<i32>()
                .unwrap_or(0x1)
        )
        .fold(0, |acc, num| acc ^ num)
        .pow(3)
        .to_string()
}
