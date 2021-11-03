use chrono::prelude::*;
use chrono::{DateTime, Duration, FixedOffset};
use serde::Serialize;

#[derive(Debug)]
pub struct Beat {
    pub beats: i16,
}

const SECONDS_PER_BEAT: f64 = 86.4; // seconds of day (usually 86400) / 1000

impl Beat {
    pub fn new(beats: i16) -> Result<Beat, &'static str> {
        if beats >= 0 && beats < 1000 {
            Ok(Beat { beats })
        } else {
            Err("beats must be between 0 and 999")
        }
    }

    pub fn now() -> Beat {
        let time: DateTime<Utc> = Utc::now();
        let timezone = FixedOffset::east(3600);
        let in_timezone = time.with_timezone(&timezone);

        Beat::with_datetime(in_timezone).unwrap()
    }

    pub fn with_datetime(datetime: DateTime<FixedOffset>) -> Result<Beat, String> {
        // TODO: check that fixed offset is UTC+01. if not, fix/err it.
        // return type Result<Beat, &str>

        let string_time = datetime.format("%H:%M:%S").to_string();
        let mut splitted_time = string_time.split(':');

        let hours = splitted_time
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("invalid hours");
        let minutes = splitted_time
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("invalid minutes");
        let seconds = splitted_time
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("invalid seconds");

        assert!(splitted_time.next().is_none());

        let beats = Beat::with_hms(hours, minutes, seconds);

        Ok(Beat { beats })
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

    pub fn time(&self) -> NaiveTime {
        let duration = Duration::seconds((f64::from(self.beats) * SECONDS_PER_BEAT) as i64);

        NaiveTime::from_hms(
            duration.num_hours() as u32 % 24,
            duration.num_minutes() as u32 % 60,
            duration.num_seconds() as u32 % 60,
        )
    }

    pub fn to_json(&self) -> String {
        let obj = BeatJSON {
            beats: self.beats,
            time: self.time(),
        };

        serde_json::to_string(&obj).unwrap()
    }
}

#[derive(Serialize)]
pub struct BeatJSON {
    pub beats: i16,
    pub time: NaiveTime,
}

pub struct BeatSwiftbarDecorator {
    pub beat: Beat,
}

impl BeatSwiftbarDecorator {
    pub fn print(&self) {
        println!("{}", self.beat.to_string());
        println!("---");
        println!(
            "{} | href={}",
            self.beat.time().format("%H:%M:%S"),
            self.url()
        );
    }

    fn url(&self) -> String {
        let datetime = Utc::now();

        format!("https://www.timeanddate.com/worldclock/fixedtime.html?day={}&month={}&year={}&beats={}&p1=0",
            datetime.day(),
            datetime.month(),
            datetime.year(),
            self.beat.beats)
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
        Beat::new(0).unwrap()
    }

    #[test]
    fn test_now() {
        let beat = Beat::now();
        let other_beats = Beat::new(beat.beats);

        assert!(other_beats.is_ok());
        assert_eq!(beat.time(), other_beats.unwrap().time());
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
    fn test_swiftbar_decorator_url() {
        let datetime = Utc::now();

        let url = format!(
            "https://www.timeanddate.com/worldclock/fixedtime.html?day={}&month={}&year={}&beats=0&p1=0",
            datetime.day(),
            datetime.month(),
            datetime.year());
        assert_eq!(BeatSwiftbarDecorator { beat: subject() }.url(), url);
    }

    #[test]
    fn test_to_json() {
        assert_eq!(
            subject().to_json(),
            format!("{{\"beats\":0,\"time\":\"{}\"}}", "00:00:00")
        );
    }

    #[test]
    fn test_with_datetime() {
        let datetime = DateTime::parse_from_rfc3339(&time_string()).unwrap();

        assert!(Beat::with_datetime(datetime).is_ok());
        assert_eq!(
            Beat::with_datetime(datetime).unwrap().time(),
            datetime.time()
        );
    }

    #[test]
    fn test_with_datetime_beats() {
        let time = DateTime::parse_from_rfc3339(&time_string()).unwrap();

        assert!(Beat::with_datetime(time).is_ok());
        assert_eq!(Beat::with_datetime(time).unwrap().beats(), 0);
    }
}
