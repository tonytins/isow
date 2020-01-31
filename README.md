# ISOW (ISO Week)

![Rust](https://github.com/tonytins/isow/workflows/Rust/badge.svg) [![Build Status](https://travis-ci.org/tonytins/isow.svg?branch=master)](https://travis-ci.org/tonytins/isow)

A CLI program that prints the local or utc week in the ISO 8601 format, e.g. 2006-W52-7. By default, ISOW uses your local time zone. Adding ``-u`` will switch to UTC.

Tom Scott's [Why It's Already 2020](https://www.youtube.com/watch?v=D3jxx8Yyw1c) inspired me to write this. This is not designed to print the full ISO 8601 date and time, only the week date. For more technical information on ISO week, [visit Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).

## Requirements

### Prerequisites

- Rust 1.41+

## Authors

- **Anthony Foxclaw** - _Initial work_ - [tonytins](https://github.com/tonytins)

See also the list of [contributors](https://github.com/tonytins/isow/contributors) who participated in this project.

## Examples

- ``isow``: ``2006-W52-7``
- ``isow -d``: ``7``
- ``isow -w``: ``W52``
- ``isow -y``: ``2006``

Year-day and week-day combinations are also possible.

- ``isow -yd``: ``2006-7``
- ``isow -wd``: ``W52-7``

## Installing

You can download binaries for any of the above support platforms by going to the [releases](https://github.com/tonytins/isow/releases) page or by installing via. Cargo by typing in ``cargo install isow``.

### Updating

You can update ISOW at anytime by typing ``isow update``. This will give you information on the latest release, if any, and any compatibility warnings with an option to accept or deny the download. Adding the ``-l`` or ``--list`` flag to the update subcommand will give you list of all available updates.

**The updater is not available in Crates.io releases.**

## License

This project is licensed under the MPL 2.0 license - see the [LICENSE](LICENSE) file for details.
