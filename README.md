# ISOW (ISO Week)

![Rust](https://github.com/tonytins/isow/workflows/Rust/badge.svg) [![Build Status](https://travis-ci.org/tonytins/isow.svg?branch=master)](https://travis-ci.org/tonytins/isow) [![Build status](https://ci.appveyor.com/api/projects/status/ffru6ik26j2b87ko?svg=true)](https://ci.appveyor.com/project/tonytins/isow) [![codecov](https://codecov.io/gh/tonytins/isow/branch/master/graph/badge.svg)](https://codecov.io/gh/tonytins/isow)

A CLI program that prints the local or utc week in the ISO 8601 format, e.g. 2006-W52-7. By default, ISOW uses your local time zone. Adding ``-u`` will switch to UTC.

Tom Scott's [Why It's Already 2020](https://www.youtube.com/watch?v=D3jxx8Yyw1c) inspired me to write this. This is not designed to print the full ISO 8601 date and time, only the week date. For more technical information on ISO week, [visit Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).

## Requirements

### Prerequisites

- Rust 1.41+
- Recommended IDEs
  - Visual Studio Code
  - Jetbrains IntelliJ

### Supported Platforms

- Ubuntu 18.04+
- Windows 10 v1809+
- macOS 10.15+

## Authors

- **Anthony Foxclaw** - _Initial work_ - [tonytins](https://github.com/tonytins)

See also the list of [contributors](https://github.com/tonytins/isow/contributors) who participated in this project.

## Installation and Usage

You can download binaries for any of the above support platforms by going to the [releases](https://github.com/tonytins/isow/releases) page or by installing via. Cargo by typing in ``cargo install isow``.

```
USAGE:
    isow [FLAGS] [SUBCOMMAND]

FLAGS:
    -d, --day        Prints the day
    -h, --help       Prints help information
    -t, --time       Prints the time
    -u, --utc        Swaps your local time zone for UTC.
    -V, --version    Prints version information
    -w, --week       Prints the week
    -y, --year       Prints the year

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    update    Updates the program to the latest version

Process finished with exit code 0
```

## Updates

You can update ISOW at anytime by typing ``isow update``. This will give you information on the latest release, if any, and any compatibility warnings with an optionto accept or deny the download.

**Self-updates are not supported in Crates.io releases.**

## License

This project is licensed under the MPL 2.0 license - see the [LICENSE](LICENSE) file for details.
