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
If you are not on any of the above distros or that method didn't work for you, you can try using the official [Rustup](https://rustup.rs) script. This script will work on virtually any GNU/Linux distro. To use this script, its recommended that you use `curl`. You probably already have it installed. If not, you can easily install it using your distro's package manager. Then run this command: 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Now just follow the prompts. You should be okay with the defaults.

## Installing other dependencies
To compile and run Strata, you need some external dependencies. These are dependencies mostly used by the [Smithay](https://github.com/smithay/smithay) library. The required dependencies are:

* `udev`
* `wayland` 
* `wayland-protocols` 
* `libinput` 
* `libxkbcommon` 
* `libglvnd` 
* `seatd` 
* `dbus-glib `
* `mesa`
* `make` (for compiling and linking)

The package names vary across different distributions. The install instructions for Arch, Fedora and Debian have been given below:

::: code-group
:::