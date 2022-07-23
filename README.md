# ISOW (ISO Week)

[![GitHub license](https://img.shields.io/github/license/tonytins/isow)](https://github.com/tonytins/isow/blob/main/LICENSE) ![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/tonytins/isow/Rust/main) ![GitHub commit activity](https://img.shields.io/github/commit-activity/w/tonytins/isow) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.0%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)

Inspired by Tom Scott's [Why It's Already 2020](https://www.youtube.com/watch?v=D3jxx8Yyw1c), ``isow`` CLI program that prints the local or utc week and time in the ISO 8601 format, e.g. 2006-W52-7T10:26:20.485371700.

This is not designed to print the full ISO 8601 date and time, only the week date. For more technical information on ISO week, [visit Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).

## Requirements

### Prerequisites

- Rust 2021 Edition or later
- IDEs or Editors
  - [Visual Studio Code](https://code.visualstudio.com/)
  - [InteliJ IDEA](https://www.jetbrains.com/idea/) or [CLion](https://www.jetbrains.com/clion/)

### Supported Platforms

- Linux 2.6.32 or later
- Windows 7 or later
- macOS 10.7 Lion or later

For more information, see Rust's [Platform Support](https://forge.rust-lang.org/release/platform-support.html) page.

## Installation and Usage

You can download binaries for any of the above support platforms by going to the [releases](https://github.com/tonytins/isow/releases) page or by installing via. Cargo by typing in ``cargo install isow``.

```text
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
```

By default, ISOW uses your local time zone. Adding ``-u`` will switch to UTC.

## Updates

You can update ISOW at anytime by typing ``isow update``. This will give you information on the latest release, if any, and any compatibility warnings with an option to accept or deny the download.

### Crates.io Differences

- Self-updater is unsupported in the Crates.io releases. As of 0.2.14, this will return a "feature is unsupported" message.

## License

I license this project under the MPL 2.0 license - see the [LICENSE](LICENSE) file for details.

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.
