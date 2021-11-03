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
        .get_matches();

    let beat = Beat::now();

    if matches.value_of("format").unwrap() == "swiftbar" {
        BeatSwiftbarDecorator { beat }.print();
    } else if matches.value_of("format").unwrap() == "json" {
        println!("{}", beat.to_json());
    } else {
        println!("{}", beat.to_string());
    }
}
