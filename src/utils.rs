use chrono::{DateTime, Utc};
use rand::Rng;

pub struct Utils;

impl Utils {
    pub fn get_date() -> String {
        let now: DateTime<Utc> = Utc::now();
        now.to_rfc3339()
    }

    pub fn get_random_int_inclusive(min: i32, max: i32) {
        let num = rand::thread_rng().gen_range(min..max);
        println!("{}", num);
    }
}

