use chrono::{DateTime, FixedOffset};

pub fn time_to_beats(time: DateTime<FixedOffset>) -> i16 {
    let string_time = time.format("%H:%M").to_string();
    let mut splitted_time = string_time.split(':');

    let hours = splitted_time.next().unwrap().parse::<i32>().unwrap();
    let minutes = splitted_time.next().unwrap().parse::<i32>().unwrap();

    let beats = f64::from((minutes + 1) * 60 + (hours + 1) * 3600) / 86.4;

    beats.floor() as i16
}
