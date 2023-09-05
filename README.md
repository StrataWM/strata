<p align=center>
    <img src="https://github.com/StrataWM/.github/blob/main/assets/profile_banner.png" />
</p>

<p align="center">
    <img src="https://img.shields.io/github/languages/top/stratawm/stratawm?style=for-the-badge"/>
    <img src="https://img.shields.io/github/commit-activity/m/stratawm/stratawm?style=for-the-badge"/>
    <img src="https://img.shields.io/github/stars/stratawm/stratawm?style=for-the-badge"/>
    <img src="https://img.shields.io/github/watchers/stratawm/stratawm.svg?style=for-the-badge"/>
    <img src="https://img.shields.io/github/license/stratawm/stratawm?style=for-the-badge"/>
</p>

## What's Strata?

Strata is a cutting-edge, robust and sleek Wayland compositor written in [Rust](https://rust-lang.org) using the [Smithay](https://github.com/smithay/smithay) library. It is designed to be minimal and flexible yet customizable. Strata is configured in [Lua](https://www.lua.org/), a lightweight, high-level, multi-paradigm programming language. Lua allows you to customize Strata to a level which may be difficult to do in other config formats such as TOML or YAML.


# Getting Started

## 1. Dependencies

To compile and use Strata, you need some dependencies that have to be installed using a package manager, such as `pacman` or `apt`, depending on your distro. The required dependencies are listed below:

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

If you're on Arch or any Arch-based distro (such as Artix, Garuda, Manjaro, etc.), you can install these using the following command:

```sh
sudo pacman -S udev wayland wayland-protocols libinput libxkbcommon libglvnd seatd dbus-glib mesa make
```

If you're on Debian, or Debian-based distros such as Ubuntu, Mate, Zorin, etc... you can install these using this command:

```sh
sudo apt-get install libudev-dev libgbm-dev libxkbcommon-dev libegl1-mesa-dev libwayland-dev libinput-dev libdbus-1-dev libsystemd-dev libseat-dev make
```

## 2. Installing Rust

To compile Strata, you have to install Rust:

### All Linux distros

```sh
curl https://sh.rustup.rs -sSf | sh
```

### Arch Linux

```sh
sudo pacman -S rust
```


## 3. Compiling
 ### 3.1 Clone the repository
 ```sh
 git clone https://github.com/stratawm/stratawm
 ```

 ### 3.2 Compile and Install
 To install, `cd` into the cloned repo and then run:
 ```sh
sudo make install
```
This will compile Strata and also copy the necessary libraries. It will also copy a default config for you to get started with. This might take a bit of time since it has to compile the source code and all the dependencies but if your system is a bit better than a potato, it won't take much time.

## Executing
To start Strata, you can run this command from a terminal

```sh
stratawm --backend winit
```

This will start Strata using the `winit` backend. For this to work, another X11 window manager or another Wayland should be running. Support for launching from the TTY will be added shortly.


# License

StrataWM and all its subsidiaries are licensed under the GNU GPL v3 License. See [LICENSE](https://github.com/stratawm/stratawm/tree/main/LICENSE) for details.

# Contributing

Refer to [CONTRIBUTING.md](https://github.com/stratawm/stratawm/tree/main/CONTRIBUTING.md)
