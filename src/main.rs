use clap::{App, Arg};
use gil_beats::Beat;
use gil_beats::BeatSwiftbarDecorator;

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
        .arg(
            Arg::with_name("beats")
                .short("b")
                .long("beats")
                .value_name("BEATS")
                .help(
                    "The beats to display (defaults to now; use with non-text format recommended)",
                )
                .takes_value(true),
        )
        .get_matches();

    let time = matches.value_of("beats").unwrap_or("now");
    let format = matches.value_of("format").unwrap_or("text");

    let beat = if time == "now" {
        Beat::now()
    } else {
        Beat::new(time.parse::<u16>().unwrap()).unwrap()
    };

    if format == "swiftbar" {
        BeatSwiftbarDecorator { beat }.print();
    } else if format == "json" {
        println!("{}", beat.to_json());
    } else {
        println!("{}", beat.to_string());
    }
}
