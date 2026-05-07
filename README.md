# ISOW (ISO Week)

A CLI program that prints the local or utc week and time in the ISO 8601 format, e.g. ``2026-W18-04``.

Tom Scott's [Why It's Already 2020](https://www.youtube.com/watch?v=D3jxx8Yyw1c) inspired me to write this. This is not designed to print the full ISO 8601 date and time, only the week date. For more technical information on ISO week, [visit Wikipedia](https://en.wikipedia.org/wiki/ISO_week_date).

## Minimum Requirements

### Development

- Swift 6.3+

### Deployment

| Target  | Version | Code Name   |
| ------- | ------- | ----------- |
| macOS   | 10.13   | High Sierra |
| Windows | 10      | Threshold   |
| Ubuntu  | 20.04   | Focal Fossa |
| Debian  | 12      | Bookworm    |
| Fedora  | 39      | N/A         |

## Usage

```
isow [-u]

OPTIONS:
  -u, --utc
```

## License

I hereby waive this project's copyright and place it the public domain - see [UNLICENSE](LICENSE) for details.
