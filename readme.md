# Dupper

Dupper is a CLI tool that helps you identify duplicate files based on their
hashes (using the [Blake3](https://crates.io/crates/blake3) hashing algorithm).

## Usage

```bash
$ dupper <PATH> -r <DEPTH>
```

Both `<PATH>` and `<DEPTH>` are optional. If you don't specify a path the
current directory will be scanned. The `-r` flag tells `dupper` to scan
reccursively, and the optional depth argument specifies how deep it should scan.

# License

This tool is made available under two different licenses. You may choose which
of these licenses you wish to use the code under.

- GNU General Public License version 3 (GPLv3):
  <https://www.gnu.org/licenses/gpl-3.0.html>

- Mozilla Public License (MPL): <https://www.mozilla.org/en-US/MPL/>
