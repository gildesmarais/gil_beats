use chrono::prelude::*;
use chrono::{DateTime, FixedOffset};
use clap::{App, Arg};
use gil_beats::Beat;

fn main() {
    let matches = App::new("gil's beats")
        .version("0.1.0")
        .author("Gil Desmarais")
        .about("A Swatch Internet Time tool, written in Rust.")
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("The format to output (text, json, swiftbar)")
                .takes_value(true)
                .default_value("text"),
        )
        .get_matches();

    let time: DateTime<Utc> = Utc::now();

    let timezone = FixedOffset::east(0);
    let in_timezone = time.with_timezone(&timezone);
    let beat = Beat::with_datetime(in_timezone);

    if matches.value_of("format").unwrap() == "swiftbar" {
        println!("{}", beat.to_string());
        println!("---");
        println!("{}", beat.datetime().format("%Y-%m-%d %H:%M:%S"));
        println!("Open URL | href={}", beat.url());
    } else if matches.value_of("format").unwrap() == "json" {
        println!("{}", beat.to_json());
    } else {
        println!("{}", beat.to_string());
    }
}
