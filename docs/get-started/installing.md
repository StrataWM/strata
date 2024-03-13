# Installing
Currently, the only way to try Strata is by compiling it using the [Rust](https://rust-lang.org) compiler.

## Installing Rust
To build Strata, you need the Rust compiler and Cargo. These maybe available as specific packages in your distro's repository. For instructions to install these on some common distros, see the [distro-specific instructions](#distro-specific). If your distro is not listed, you can try [this](#using-the-rustup-script).

### Distro-specific

::: code-group

```bash [Arch Linux]
sudo pacman -S rust
```

```bash [Fedora]
sudo dnf install rust cargo
```

```bash [Debian (based)]
sudo apt install rustc cargo
```

```bash [openSUSE]
sudo zypper install rust cargo
```

```bash [Void Linux]
sudo xbps-install -S rust cargo
```

```bash [Gentoo]
sudo emerge dev-lang/rust sys-devel/cargo
```
:::

### Using the `rustup` script
If you are not on any of the above distros or that method didn't work for you, you can try using the official Rustup script