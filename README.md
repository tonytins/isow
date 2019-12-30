# ISOW (ISO Week)

ISO Week is a simple CLI program that prints the local date and time in the ISO 8601 format. Tom Scott's [Why It's Already 2020](https://www.youtube.com/watch?v=D3jxx8Yyw1c) inspired me to write this.

## Requirements

### Prerequisites

-   Rust 2018 update

## Authors

- **Anthony Foxclaw** - _Initial work_ - [tonytins](https://github.com/tonytins)

See also the list of [contributors](https://github.com/tonytins/isow/contributors) who participated in this project.

## Examples

- ``isow``: ``2020-W01``
- ``iso -w``: ``W1``
- ``iso -y``: ``2020``

Adding ``-u`` swaps your local time for UTC. However, ``-y`` and ``-w`` can't be used together to give you the same output as running ``isow`` on it's own. This will be fixed.

## License

This project is licensed under the MPL 2.0 license - see the [LICENSE](LICENSE) file for details.