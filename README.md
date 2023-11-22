# jd

A command line interface for interacting with Johnny Decimal systems.

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
```

## Contributing

This software should be bug free, however contributions are welcome. Remember to write tests for any new functionality and ensure that all existing tests pass.

## Todo

- [ ] jd add: Add a new ID to an area/category
- [ ] jd rm/remove: Remove an ID and change all existing numbers?
- [ ] jd mv/move: Move an area/category and rename all child IDs
- [ ] jd ls: list area/category
- [ ] Fish completions for jd directories
