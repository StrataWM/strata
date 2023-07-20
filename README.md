<p align="center">
    <h1 align="center">Strata</h1>
    <p align="center">
        <img src="https://github.com/StrataWM/.github/blob/main/assets/strata_logo.png" style="width: 25%;"></img>
        <h3 align="center">A Modular, Sleek and Dynamic Wayland Compositor with batteries included!</h3>
    </p>
</p>

# How it works
Strata is made to be modular. It is done this way so that you, the user can mix'n'match different components to make Strata work the way you want. Strata follows this architecture, which is inspired by [BSPWM](https://github.com/baskerville/bspwm):

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
First you have to clone this repository. Run
```
git clone https://github.com/stratawm/stratawm
```
Then change into the cloned directory using `cd stratawm`

## Install using the script
There is a script which allows you to install both StrataWM and StrataCTL at once. Just run this command
```
./install.sh
```

## Running Strata
We can use `stratactl` to launch Strata. First start the server by running 
```sh
stratawm &
```
Then run this
```
stratactl launch winit
```
This will launch Strata using the Winit backend. You can create an alias to start Strata using one command. Just create an alias to this command in your shell
```
stratawm &; sleep 1; stratactl launch winit;
```
The `sleep 1` is required because otherwise, the shell will execute the second command before the server is started, resulting in an error. You can put this in a file and add it to your $PATH variable. In the file, you can also set other environment variables before starting Strata. You can also add your startup programs to that. Its kinda like an `.xinitrc` file.

## Running locally
To just test Strata without installing it, run

```sh
cargo run
```
**NOTE:** You need StrataCTL to be able to control Strata. Without it, Strata is unusable.

# License
StrataWM and all its subsidaries are licensed under the GNU GPL v3 License. See [LICENSE](https://github.com/stratawm/stratawm/tree/main/LICENSE) for details.

# Contributing
If you want to contribute to Strata, then create a fork, make your changes and open a pull request. While making any changes, please format your code using the config in the repo ([rustfmt.toml](https://github.com/stratawm/stratawm/tree/main/rustfmt.toml)). A Contributing.md file will be created shortly