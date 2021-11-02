# gil_beats

One day after we were given an hour as a (to-be-returned) gift (change from summer time to winter time, daylight saving time (DST)), there was a small discussion on Twitter about when the European Union would finally abolish daylight saving time.

Of course, someone mentioned [Swatch Internet Time](https://en.wikipedia.org/wiki/Swatch_Internet_Time), a time-zone-less time format from 1998. The idea - that of computer scientist [Nicholas Negroponte](https://en.wikipedia.org/wiki/Nicholas_Negroponte) - was unsuccessful.

Unimpressed by all this, I thought a program that will help me deal with the Internet Time could be a nice little project to invest the hour well and learn some Rust. Here it is.

Since I have no intention of making it a full-fledged crate, and due to lack of name-finding creativity, I have prefixed the obvious name `beats` with my first name and thus called it `gil_beats`.

## Usage & Development

1. Install Rust (e.g. via your version manager or `brew install rust` or other methods)
2. `git clone https://github.com/gildesmarais/gil_beats.git` and `cd` into it
3. To run: `cargo run`
4. To install: `cargo install --path .`, check for warnings and act, eventually run it: `gil_beats`

### Use with Bitbar/XBar/Swiftbar

Install gil_beats (see instructions above).

Create a shell script in your \*bar plugins folder, called e.g. `gil_beats.87s.sh`:

```sh
#!/bin/sh

"$HOME/.cargo/bin/gil_beats" --format=swiftbar
```

and `chmod +x gil_beats.87s.sh`.

## TODO

- [x] Start with printing Time now
- [x] create method to return a beat
- [x] just print the current beat.
- [x] use the correct time zone
- [x] useable as lib
- [x] Output --format:
  - [x] text (default)
  - [x] swiftbar (with clickable with url)
  - [x] json
- [x] add github actions for ci/cd
- [x] add tests
- [ ] publish as a crate
- [ ] provide install option via homebrew
- [x] add a (bit|x|swift)bar script example to this README
