# timestamp-prefix

`timestamp-prefix` is a command line utility to prefix filenames with timestamps.

## Example

```sh
$ timestamp-prefix --dry-run foo bar
mv "foo" "250503_065840_foo"
mv "bar" "250503_065840_bar"
```

## Usage

```
Usage: timestamp-prefix [OPTIONS] [FILE]...

Arguments:
  [FILE]...


Options:
  -n, --dry-run
          Don't actually do anything

  -v, --verbose
          Print what happens

  -k, --kind <KIND>
          Which timestamp to use

          [default: created]

          Possible values:
          - created
          - modified
          - earliest: created or modified, whichever is smaller
          - latest:   created or modified, whichever is larger

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
