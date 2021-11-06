# gil_beats

This command-line program deals with the [Swatch Internet Time](https://en.wikipedia.org/wiki/Swatch_Internet_Time).

Features:

- calculates the current _@beats_
- supports _@beats_ to `H:m:s` time format
- outputs in several formats:
  - Bitbar/Xbar/Swiftbar
  - JSON
  - raw text

It's also usable in other Rust programs as a library.

## CLI examples

Supports several output formats:

```
$ gil_beats
@469

$ gil_beats --format=json
{"beats":469,"time":"11:15:21"}

$ gil_beats --format=swiftbar
@469
---
11:15:21 | href=https://www.timeanddate.com/worldclock/fixedtime.html?day=6&month=11&year=2021&beats=469&p1=0
```

Beats to time:

```
$ gil_beats --beats 500 --format json
{"beats":500,"time":"12:00:00"}
```

## History

One day, after we had been given an hour (change from summer time to winter time, daylight saving time (DST)), there was a small discussion on Twitter about when the European Union would finally implement its plans to abolish daylight saving time.

Someone mentioned Swatch Internet Time, a time-zone-less time format from 1998. The idea was unsuccessful.

Unimpressed by all this, I thought a program that will help me deal with the Internet Time could be a nice little project to invest the hour well and learn some Rust. Here it is.

Since I have no intention of making it a full-fledged crate, and due to lack of name-finding creativity, I have prefixed the obvious name `beats` with my first name and hence called it `gil_beats`.

## Usage & Development

1. Install Rust (e.g. rustup, via your version manager or `brew install rust` or which method you prefer)
2. `git clone https://github.com/gildesmarais/gil_beats.git` and `cd` into it
3. To run: `cargo run -- --help`
4. To install: `cargo install --path .`, check for warnings and act, eventually run it: `gil_beats`

### Use with Bitbar/XBar/Swiftbar...

To have the current _@beats_ displayed in your menu bar, use:

- <https://github.com/matryer/xbar> or
- <https://github.com/swiftbar/SwiftBar>

and install gil_beats (see instructions above). Then, create this script in your
\*bar plugins folder, called e.g. `gil_beats.87s.sh`:

```sh
#!/bin/sh

"$HOME/.cargo/bin/gil_beats" --format=swiftbar

# or:
# gil_beats --format=swiftbar
```

Finally, `chmod +x gil_beats.87s.sh` and a new item on your menu bar should appear.
