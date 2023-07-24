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
Strata is a dynamic and sleek Wayland compositor and window manager for GNU/Linux systems. It's written completely in [Rust](https://rust-lang.org) using the [Smithay](https://github.com/smithay/smithay) library. Strata is made to be modular. It is done this way so that you, the user can mix N match different components to make Strata work the way you want. Strata follows this architecture, which is inspired by [BSPWM](https://github.com/baskerville/bspwm):
```
╭───────────────╮       ╭───────────╮       ╭──────────╮
│ Hotkey Daemon │ ────> │ StrataCTL │ ────> │ StrataWM │
╰───────────────╯       ╰───────────╯       ╰──────────╯
```

This makes it possible so that you can interchange components, for example, instead of using Kagi, the official Strata hotkey daemon, you can use [SWHKD](https://github.com/waycrate/swhkd) or even make your own. As Strata grows in complexity, this architecture will, hopefully, prove to be useful. 

# Getting Started
## Installing Rust
To use Strata, you have to install Rust:

### All Linux distros
```sh
curl https://sh.rustup.rs -sSf | sh
```
### Arch Linux
```sh
sudo pacman -S rust
```

## Clone this repo
First, you have to clone this repository. Run
```
git clone https://github.com/stratawm/stratawm
```
Then change into the cloned directory using `cd stratawm`

## Install using the script
There is a script that allows you to install both StrataWM and StrataCTL at once. Just run this command
```
./install.sh
```

## Running Strata
We can use `stratactl` to launch Strata. 
```
stratawm -b winit
```
This will launch Strata using the Winit backend. If you want, you could create a shell file and then set all your environment variables before starting Strata.

## Running locally
To just test Strata without installing it, run

```sh
cargo run
```
**NOTE:** You need StrataCTL to be able to control Strata. Without it, Strata is unusable.

# License
StrataWM and all its subsidiaries are licensed under the GNU GPL v3 License. See [LICENSE](https://github.com/stratawm/stratawm/tree/main/LICENSE) for details.

# Contributing
If you want to contribute to Strata, then create a fork, make your changes and open a pull request. While making any changes, please format your code using the config in the repo ([rustfmt.toml](https://github.com/stratawm/stratawm/tree/main/rustfmt.toml)). A Contributing.md file will be created shortly
