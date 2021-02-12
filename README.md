# utils
Utils for every days

## readbin

Some utils to read binary header like `elf64`
Want to replace `readelf` with a generic tools for all binary from `elf` to `mach-o`

## Hex

Some utils to convert hexdecimal function
In progress but not a lot of time to use on this

```
hex 1.0
LSH. <github@lsh.tech>
Does awesome things with hexadecimal

USAGE:
    hex [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help            Prints this message or the help of the given subcommand(s)
    substraction    substract to hex
    to              convert hex to ...
      SUBCOMMANDS:
          be            convert hex to big endian
          be_payload    convert hex to an big endian payload
          decimal       convert hex to decimal
          help          Prints this message or the help of the given subcommand(s)
          le            convert hex to big endian

```
