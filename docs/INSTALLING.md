# Installing `jd`

`jd` has official support for NixOS.

## [NixOS](https://nixos.wiki/wiki/Overview_of_the_NixOS_Linux_distribution) (Recommended)

> TODO: `jd` has not reached 0.1.0 yet.

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

## Other distributions

Since `jd` embraces the Rust ecosystem, it's easy to get working on any operating system.

If you want to contribute installation instructions for your OS, feel free to open a pull request.
