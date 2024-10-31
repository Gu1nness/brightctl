# brightctl -- manage your screen brightness from CLI

syntax is `{number}{optional %}{optional +-}`

So:

* `5%+` adds 50% of max brightness.
* `50` sets absolute value of 50.
* `50+` adds 50 to current value.
* `5%` sets to 5% of max brightness.

```
Usage: brightctl [OPTIONS] [COMMAND]

Commands:
  i / info
  g / get
  m / max
  s / set
  help  Print this message or the help of the given subcommand(s)

Options:
  -l, --list
      --pretend
  -m, --machine-readable
  -n, --min-value <MIN_VALUE>  N{%}{+-} [default: 1]
  -d, --device <DEVICE>
  -c, --class <CLASS>
  -h, --help                   Print help
  -V, --version                Print version
```

Some udev rules are provided to be able to use this cli tool.
