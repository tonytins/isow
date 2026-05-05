# Change Log

## 0.3

Rewrote in Swift. This new version no longer carries all the baggage from the original Rust implantation did. While this certainly has it's uses, it wasn't meant to be anything *that* serious. I just simply made it for fun. This project only evolved the way it did because it required so much extra dependencies that required extra oversight.

## Rust

### 0.2.15

- If Git isn't present, show only the version without the hash and dirty/clean suffixes. This is meant for Crates.io builds.

### 0.2.13

- Fixed an oversight where the ``--utc`` flag returned the local time.
- If the updater flag is not enabled, ISOW will return "feature not supported".

### 0.2.11

- Version now includes the git commit hash. This should help improve security, as described on the [Reproducible Builds](https://reproducible-builds.org/) website, and provide a good means of debugging production builds. Versions prior to 0.2.11 should be considered untrusted going forward.

### 0.2.10

- Time has been added shown default along with the rest of the date.  It can be optionally added with the ``-t`` or ``--time`` flag in combo with any existing ones.

### 0.2.6

- Maintenance releases. Internal changes.

## 0.2.5

- Removed arbitrary flag conflicts and allowed support for all possible combos, including the default. The reason they were there in the first place was due to my misunderstanding of how Rust interrupts match patterns.
- Big version bump to 0.2.5, due to how usefulness factor going way up.
- Changed the match pattern order to better reflect the actual iso week format.
- Renamed output artifact
- Updated dependencies

## 0.2.1

- Renamed "status" flag to "list" in the update subcommand

## 0.2

- Relatively stable features
- Initial release with self-updating support.

## 0.1

This was the internal development version that focused on getting it to a relatively stable release.
