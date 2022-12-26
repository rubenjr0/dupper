[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua)

# Dupper

![Crates.io](https://img.shields.io/crates/v/dupper?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/l/dupper?style=for-the-badge)
![GitHub Repo stars](https://img.shields.io/github/stars/rubenjr0/dupper?style=for-the-badge)

Dupper is a CLI tool that helps you identify duplicate files based on their
hashes (using the [Seahash](https://crates.io/crates/seahash) hashing
algorithm).

## Installation

You can install dupper by using cargo:

```bash
$ cargo install dupper
```

## Usage

```bash
$ dupper <PATH> -r <DEPTH>
```

Both `<PATH>` and `<DEPTH>` are optional. If you don't specify a path the
current directory will be scanned. The `-r` flag tells `dupper` to scan
reccursively, and the optional depth argument specifies how deep it should scan.

# License

This tool is made available under
[GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.html).
