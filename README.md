# ISOW (ISO Week)

![Rust](https://github.com/tonytins/isow/workflows/Rust/badge.svg)

A CLI program that prints the local or utc week in the ISO 8601 format, e.g. 2006-W52-7.

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

By default, ISOW uses your local time zone. Adding ``-u`` will switch to UTC.

## Updating

You can update ISOW at anytime by typing ``isow update``. This will give you information on the latest release, if any, and any compatibility warnings with an optionto accept or deny the download. Adding the ``-l`` or ``--list`` flag to the update subcommand will give you list of all available updates.

## License

This project is licensed under the MPL 2.0 license - see the [LICENSE](LICENSE) file for details.
