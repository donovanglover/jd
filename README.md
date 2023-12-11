# jd

A CLI and web server for interacting with [Johnny Decimal](https://johnnydecimal.com/) systems.

## Features

- Automatically rename IDs on area/category changes.

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

### [Arch Linux](https://archlinux.org/)

```fish
git clone https://github.com/donovanglover/jd -b 0.1.0 && cd jd && makepkg -si
```

### Other distributions

Follow the [install guide](https://www.rust-lang.org/tools/install) for Rust. Then, use cargo to install jd.

```fish
cargo install --git https://github.com/donovanglover/jd --tag 0.1.0
```

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

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Todo

- [ ] jd add: Add a new ID to an area/category
- [ ] jd rm/remove: Remove an ID and change all existing numbers?
- [ ] jd mv/move: Move an area/category and rename all child IDs
- [ ] jd ls: list area/category
- [ ] Fish completions for jd directories
- [ ] server for index notes / browse johnny decimal system remotely
- [ ] validate johnny decimal systems to ensure that regular files aren't stored outside of ids
