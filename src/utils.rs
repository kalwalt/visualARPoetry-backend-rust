use chrono::{DateTime, Utc};

pub struct Utils;

impl Utils {
    pub fn get_date() -> String {
        let now: DateTime<Utc> = Utc::now();
        now.to_rfc3339()
    }
}

