use chrono::Datelike;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;

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
        assert!(splitted_time.next().is_none());

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

#[cfg(test)]
mod tests {
    use super::*;

    const TIME_STRING: &str = "2021-11-01T00:00:00+01:00";

    #[test]
    fn test_to_string() {
        let beat = subject();
        assert_eq!(beat.to_string(), "@42");
    }

    #[test]
    fn test_with_datetime() {
        let time = DateTime::parse_from_rfc3339(&TIME_STRING).unwrap();

        assert_eq!(Beat::with_datetime(time).beats(), 42);
    }

    #[test]
    fn test_url() {
        assert_eq!(
            subject().url(),
            "https://www.timeanddate.com/worldclock/fixedtime.html?day=1&month=11&year=2021&beats=42&p1=0"
        );
    }

    #[test]
    fn test_to_json() {
        assert_eq!(
            subject().to_json(),
            format!("{{\"beats\":42,\"datetime\":\"{}\"}}", TIME_STRING)
        );
    }

    #[test]
    fn test_datetime() {
        let time = DateTime::parse_from_rfc3339(&TIME_STRING).unwrap();

        assert_eq!(Beat::with_datetime(time).datetime(), time);
    }

    fn subject() -> Beat {
        let time = DateTime::parse_from_rfc3339(&TIME_STRING).unwrap();

        Beat::with_datetime(time)
    }
}
