# `yoink`

_**yoink**_ all kinds of information straight out of your system

no one can tell you what to do

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

When a file is yoinked it stores the yoinked information right next to the yoinkfile.
If you have a yoinkfile named `hello.txt.yoink` it will yoink into a file in the same directory called `hello.txt`.

### `hello.txt.yoink` - yoink full files
Any files full bytes can be yoinked.
```toml
[target.bytes]
# relative paths are resolved relative to the yoinkfile itself
path = "./relative/path/to/hello.txt"
```

### `dconf.ini.yoink` - yoink dconf data
Dconf databases can be yoinked by selecting specific prefixes.
```toml
[target.dconf]
path = "/home/rhedgeco/.config/dconf/user"
include = ["/org/gnome/desktop"]
```
