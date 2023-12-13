# jd

CLI, web server, and library for interacting with [Johnny Decimal](https://johnnydecimal.com/) systems.

## Features

- Automatically rename IDs on area/category changes.

## Installation

See [INSTALLING.md](docs/INSTALLING.md)

## Usage

```man
Usage: jd [OPTIONS] [COMMAND]

Commands:
  add     Add a new area, category, or id
  remove  Remove an existing area, category, or id
  ls      List the contents of an area, category, or id
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Print debugging information
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

## Contributing

See [CONTRIBUTING.md](docs/CONTRIBUTING.md)

## Todo

- [ ] jd add: Add a new ID to an area/category
- [ ] jd rm/remove: Remove an ID and change all existing numbers?
- [ ] jd mv/move: Move an area/category and rename all child IDs
- [ ] jd ls: list area/category
- [ ] Fish completions for jd directories
- [ ] server for index notes / browse johnny decimal system remotely
- [ ] validate johnny decimal systems to ensure that regular files aren't stored outside of ids
