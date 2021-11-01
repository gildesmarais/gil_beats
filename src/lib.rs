use chrono::{DateTime, FixedOffset};
use chrono::Datelike;
use serde::{Serialize};

#[derive(Serialize)]
pub struct Beat {
    pub beats: i16,
    pub datetime: DateTime<FixedOffset>,
}

impl Beat {
    pub fn with_datetime(datetime: DateTime<FixedOffset>) -> Beat {
        let string_time = datetime.format("%H:%M").to_string();
        let mut splitted_time = string_time.split(':');

        let hours = splitted_time.next().unwrap().parse::<i32>().unwrap();
        let minutes = splitted_time.next().unwrap().parse::<i32>().unwrap();

        let beats = (f64::from((minutes + 1) * 60 + (hours + 1) * 3600) / 86.4).floor() as i16;

        Beat { beats, datetime }
    }

    pub fn beats(&self) -> i16 {
        self.beats
    }

    pub fn datetime(&self) -> DateTime<FixedOffset> {
        self.datetime
    }

    pub fn to_string(&self) -> String {
        format!("@{}", self.beats)
    }

    pub fn url(&self) -> String {
        format!("https://www.timeanddate.com/worldclock/fixedtime.html?day={}&month={}&year={}&beats={}&p1=0",
        self.datetime().day(),
        self.datetime().month(),
        self.datetime().year(),
        self.beats)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
