use chrono::prelude::*;
use chrono::{DateTime, FixedOffset};

fn main() {
    let utc: DateTime<Utc> = Utc::now();

    let timezone = FixedOffset::east(0);
    let in_timezone = utc.with_timezone(&timezone);

    println!("@{}", gil_beats::time_to_beats(in_timezone));
}
