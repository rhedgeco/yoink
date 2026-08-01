# `yoink`

_**yoink**_ all kinds of information straight out of your system. no one can tell you what to do.

## Usage

```shell
$ yoink --help
Usage: yoink [OPTIONS] [TARGET]

Arguments:
  [TARGET]  Target file or directory to be yoinked (Defaults to current dir)

Options:
  -r, --recursive  Recurse subdirectories
  -h, --help       Print help
  -V, --version    Print version
```

## Configuration

Yoink is controlled through yoinkfiles.
A yoinkfile is anything with a `*.yoink` extension.

When a file is yoinked it stores the yoinked information in the same directory as the yoinkfile. \
If you have a yoinkfile named `hello.txt.yoink` it will yoink information into a file in the same directory called `hello.txt`.

A yoinkfile is written using toml, and always starts with `target = "type"`.
Anything after that is configuration associated with the specific `type`.

### `hello.txt.yoink` - yoink full files

This yoinks the entire byte content at the location specified by the `path` paramaeter.

```toml
target = "bytes"
path = "./relative/path/to/hello.txt"
```

### `dconf.ini.yoink` - yoink dconf data

Dconf database pairs can be yoinked by selecting specific prefixes.

Key value pairs can be included if the key starts with one of the strings in the `include` list. \
Items that need to be excluded can also be specified in the `exclude` list.

```toml
target = "dconf"
path = "/home/rhedgeco/.config/dconf/user"
include = ["/org/gnome/desktop"]
exclude = ["/org/gnome/desktop/interface/gtk-theme"]
```
