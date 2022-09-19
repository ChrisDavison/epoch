# epoch

Convert seconds since epoch to readable date

## Usage

This is a very simple utility. 

- Required argument `EPOCH` -- seconds since unix epoch (`1970-01-01 00:00:00`).
- Optional argument `-f|--foramt <FMT>` is the time format string for output, default `"%Y-%m-%d %H:%M:%S"`

```
epoch

USAGE:
    epoch [OPTIONS] <EPOCH>

ARGS:
    <EPOCH>    Seconds since unix epoch

OPTIONS:
    -f, --format <FORMAT>    Format for time output [default: "%Y-%m-%d %H:%M:%S"]
    -h, --help               Print help information
```

## Example

- `epoch 0 -f "%Y-%m-%d"` prints `1970-01-01`
- `epoch 1663625525` prints `2022-09-19 22:12:05`
