use rocket::post;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Debug, Serialize, Deserialize)]
pub struct Raindeer {
    name: String,
    strength: u32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RaindeerExtended {
    name: String,
    strength: u32, 
    speed: f32,
    height: u32,
    antler_width: u32, 
    snow_magic_power: u32,
    favorite_food: String, 
    #[serde(rename(serialize = "cAnD13s_3ATeN-yesT3rdAy", deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candy_eaten_yesterday: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContestResponse {
    fastest: String,
    tallest: String,
    magician: String, 
    consumer: String,
}

#[post("/4/strength", format = "json", data = "<raindeers>")]
pub fn strength(raindeers: Json<Vec<Raindeer>>) -> String {
    println!("raindeers = {:#?}", raindeers);
    println!("sum = {}", raindeers.iter()
        .fold(0, |accum, raindeer| accum + raindeer.strength)
        .to_string()
    );
    raindeers.iter()
        .fold(0, |accum, raindeer| accum + raindeer.strength)
        .to_string()
}

#[post("/4/contest", format =  "json", data = "<raindeers>")]
pub fn contest(raindeers: Json<Vec<RaindeerExtended>>) -> Json<ContestResponse> {
    let mut fastest = &raindeers[0];
    let mut tallest = &raindeers[0];
    let mut magician = &raindeers[0];
    let mut consumer = &raindeers[0];
    for raindeer in raindeers.iter() {
        if raindeer.speed > fastest.speed {
            fastest = raindeer;
        }
        if raindeer.height > tallest.height {
            tallest = raindeer;
        }
        if raindeer.snow_magic_power > magician.snow_magic_power {
            magician = raindeer;
        }
        if raindeer.candy_eaten_yesterday > consumer.candy_eaten_yesterday {
            consumer = raindeer;
        }
    }

    Json( 
        ContestResponse {
        fastest: format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name),
        tallest: format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width),
        magician: format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power),
        consumer: format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food),
    })
}