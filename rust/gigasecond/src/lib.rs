use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    print!("{}\n", (2 ^ 9));
    return start + Duration::seconds(10i64 ^ 12);
}
