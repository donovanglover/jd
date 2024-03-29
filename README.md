# jd

CLI and library for interacting with [Johnny.Decimal](https://johnnydecimal.com/) systems.

## Features

- Automatically rename IDs on area/category changes.
- Non-destructive.
  - Your files are never deleted, only trashed.
  - Your files are never overwritten. New folders are only created if they don't exist already.
- Use your favorite note editor.
  - TODO: Support for building indexes from a custom ruleset? Make no assumptions and let user manually create rules? Have a set of predefined rules?

## Installation

### [NixOS](https://nixos.wiki/wiki/Overview_of_the_NixOS_Linux_distribution) (Recommended)

Add [`jd`](https://search.nixos.org/packages?channel=unstable&query=jd) to your `systemPackages` and rebuild.

```nix
{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    jd
  ];
}
```

Alternatively, use `nix run nixpkgs#jd` to try jd without installing it.

### Other distributions

Follow the [install guide](https://www.rust-lang.org/tools/install) for Rust. Then, use cargo to install jd.

```fish
git clone https://github.com/donovanglover/jd && cd jd && cargo install --path jd
```

## Usage

```man
Usage: jd [OPTIONS] [COMMAND]

Commands:
  add     Add a new area, category, or id
  remove  Remove an existing area, category, or id
  index   List the contents of an area, category, or id
  insert  Insert a new area, category, or id, moving existing ones
  clean   Checks for empty folders and missing areas/categories/ids (Prune/Clean)
  serve   Start a new web server
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --dir <DIR>  Directory where areas are stored [default: $HOME]
  -v, --verbose    Print debugging information
  -h, --help       Print help (see more with '--help')
  -V, --version    Print version
```

## Contributing

Contributions are welcome. Make new issues if you have feature requests for the library or CLI.

## Todo

- [x] jd add: Adds a new area/category/id to the index
- [x] jd rm: Removes an existing area/category/id from the index
- [ ] jd mv/move: Move an area/category and rename all child IDs
- [x] jd index: pretty print index
- [ ] Fish completions for jd directories
- [ ] server for index notes / browse johnny decimal system remotely
- [ ] validate johnny decimal systems to ensure that regular files aren't stored outside of ids
- [ ] program a way to guarantee the positioning of comments?
- [ ] support for dmenu?
- [ ] jd serve <id> to serve a note or directory for a specific id?
