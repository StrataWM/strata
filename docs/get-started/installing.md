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

```bash [Arch Linux]
sudo pacman -S udev wayland wayland-protocols libinput libxkbcommon libglvnd seatd dbus-glib mesa make
```

```bash [Fedora]
sudo dnf install systemd-devel libgbm-devel libxkbcommon-devel Mesa-libEGL-devel wayland-devel libinput-devel dbus-glib-devel libseat-devel
```

```bash [Debian/Ubuntu/Pop!_OS]
sudo apt-get install libudev-dev libgbm-dev libxkbcommon-dev libegl1-mesa-dev libwayland-dev libinput-dev libdbus-1-dev libsystemd-dev libseat-dev make
```
:::

## Installing Strata
Now, to install Strata, you first have to clone the Git repository. For this, using `git` is recommended. However, you can also just download the Zip archive and extract it. 

::: details Why Git?
Since Strata is being constantly updated, you'll often have to download the latest repository and re-compile it. Using `git` makes this significantly easier than downloading and extracting the Zip archive each time.
:::

To clone the repo, run:

```bash
git clone https://github.com/stratawm/strata
 ```

Then to install Strata, `cd` into the cloned repo:

```bash
cd strata
```

and run this command to install it:

```bash
sudo make install
```

If this command finishes without any errors, then Strata has been installed successfully :tada:!

## Updating Strata
Strata is being constantly updated. New features are added and bugs are fixed. To update your local installation, follow these steps.

First `cd` into the directory where you initially clone the repo:

```bash
cd strata
```

Next, you need to configure Git to rebase the branch when pulling. To do this, run:

```bash
git config pull.rebase true
```

Now you can run:

```bash
git pull
```

This will fetch all the latest changes from the remote repo. Finally you can run

```bash
sudo make install
```

to recompile and install Strata with the latest changes.

If you face any issues while installing Strata, checkout out the next page which has solutions for many common issues.