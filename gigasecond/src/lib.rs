use chrono::{DateTime, Utc, Duration};
//use time::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // The bilion is 2 to 9 but it not 2 to 12 in this case!
    return start + Duration::seconds(1_000_000_000)
}
