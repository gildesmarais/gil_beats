use chrono::prelude::*;
use chrono::{DateTime, Datelike, Duration, FixedOffset};
use serde::Serialize;

#[derive(Debug)]
pub struct Beat {
    pub beats: i16,
}

#[derive(Serialize)]
struct BeatSerialized {
    pub beats: i16,
    pub datetime: DateTime<FixedOffset>,
}

const SECONDS_PER_BEAT: f64 = 86.4; // seconds of day (usually 86400) / 1000

impl Beat {
    pub fn new(beats: i16) -> Beat {
        Beat { beats }
    }

    pub fn now() -> Beat {
        let time: DateTime<Utc> = Utc::now();
        let timezone = FixedOffset::east(3600);
        let in_timezone = time.with_timezone(&timezone);

        Beat::with_datetime(in_timezone)
    }

    pub fn with_datetime(datetime: DateTime<FixedOffset>) -> Beat {
        // TODO: check that fixed offset is UTC+01. if not, fix/err it.
        // return type Result<Beat, &str>

        let string_time = datetime.format("%H:%M:%S").to_string();
        let mut splitted_time = string_time.split(':');

        let hours = splitted_time.next().unwrap().parse::<i32>().unwrap();
        let minutes = splitted_time.next().unwrap().parse::<i32>().unwrap();
        let seconds = splitted_time.next().unwrap().parse::<i32>().unwrap();

        assert!(splitted_time.next().is_none());

        let beats = Beat::with_hms(hours, minutes, seconds);

        Beat { beats }
    }

    fn with_hms(hours: i32, minutes: i32, seconds: i32) -> i16 {
        let seconds_of_day: i32 = seconds + minutes * 60 + hours * 3600;

        (f64::from(seconds_of_day) / SECONDS_PER_BEAT).floor() as i16
    }

    pub fn beginning_of_day() -> i16 {
        Beat::with_hms(0, 0, 0)
    }

    pub fn end_of_day() -> i16 {
        Beat::with_hms(23, 59, 59)
    }

    pub fn beats(&self) -> i16 {
        self.beats
    }

    pub fn to_string(&self) -> String {
        format!("@{:03}", self.beats)
    }

    pub fn datetime(&self) -> DateTime<FixedOffset> {
        let duration = Duration::seconds((f64::from(self.beats) * SECONDS_PER_BEAT) as i64);

        let time_string = format!(
            "{}T{:02}:{:02}:{:02}+01:00",
            Utc::now().format("%Y-%m-%d").to_string(),
            duration.num_hours() % 24,
            duration.num_minutes() % 60,
            duration.num_seconds() % 60
        );

        let datetime = DateTime::parse_from_rfc3339(&time_string);

        match datetime {
            Ok(dt) => dt,
            Err(err) => panic!("Can't parse time_string {}, error: {:?}", time_string, err),
        }
    }

    pub fn url(&self) -> String {
        format!("https://www.timeanddate.com/worldclock/fixedtime.html?day={}&month={}&year={}&beats={}&p1=0",
        self.datetime().day(),
        self.datetime().month(),
        self.datetime().year(),
        self.beats)
    }

    pub fn to_json(&self) -> String {
        let obj = BeatSerialized {
            beats: self.beats,
            datetime: self.datetime(),
        };

        serde_json::to_string(&obj).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time_string() -> String {
        format!(
            "{}T{:02}:{:02}:{:02}+01:00",
            Utc::now().format("%Y-%m-%d").to_string(),
            0,
            0,
            0
        )
    }

    fn subject() -> Beat {
        Beat::new(0)
    }

    #[test]
    fn test_now() {
        let beat = Beat::now();

        assert_eq!(beat.datetime(), Beat::new(beat.beats).datetime());
    }

    #[test]
    fn test_with_hours_and_minutes() {
        assert_eq!(Beat::with_hms(0, 0, 0), 0);
        assert_eq!(Beat::with_hms(0, 2, 0), 1);

        assert_eq!(Beat::with_hms(6, 0, 1), 250);
        assert_eq!(Beat::with_hms(12, 0, 1), 500);
        assert_eq!(Beat::with_hms(18, 0, 1), 750);

        assert_eq!(Beat::with_hms(24, 0, 0), 999);

        // TODO: handle these 'false' values appropiately
        assert_eq!(Beat::with_hms(0, 0, 87), 1);
        assert_eq!(Beat::with_hms(24, 0, 1), 1000);
    }

    #[test]
    fn test_beginning_of_day() {
        assert_eq!(Beat::beginning_of_day(), 0);
    }

    #[test]
    fn test_end_of_day() {
        assert_eq!(Beat::end_of_day(), 999);
    }

    #[test]
    fn test_to_string() {
        let beat = subject();
        assert_eq!(beat.to_string(), "@000");
    }

    #[test]
    fn test_url() {
        let datetime = subject().datetime();

        let url = format!(
            "https://www.timeanddate.com/worldclock/fixedtime.html?day={}&month={}&year={}&beats=0&p1=0",
            datetime.day(),
            datetime.month(),
            datetime.year());
        assert_eq!(subject().url(), url);
    }

    #[test]
    fn test_to_json() {
        assert_eq!(
            subject().to_json(),
            format!("{{\"beats\":0,\"datetime\":\"{}\"}}", time_string())
        );
    }

    #[test]
    fn test_with_datetime() {
        let time = DateTime::parse_from_rfc3339(&time_string()).unwrap();

        assert_eq!(Beat::with_datetime(time).datetime(), time);
    }

    #[test]
    fn test_with_datetime_beats() {
        let time = DateTime::parse_from_rfc3339(&time_string()).unwrap();

        assert_eq!(Beat::with_datetime(time).beats(), 0);
    }
}
