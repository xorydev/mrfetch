
# mrfetch
![GitHub](https://img.shields.io/github/license/iVacon/mrfetch?style=flat-square) ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/iVacon/mrfetch/rust.yml?style=flat-square) ![GitHub top language](https://img.shields.io/github/languages/top/iVacon/mrfetch?style=flat-square)

This is the repository for ``mrfetch``. ``mrfetch`` is a fetch utility created by iVacon as a nitch-inspired hobby project made in Rust. The ``mrfetch`` tool is in no way, shape, or form, endorsed by the Rust Foundation or Project. Disclaimers aside, this started as nothing more but a personal hobby project for me (iVacon) to practice Rust with. I don't expect this to get popular, and definitely not to replace nitch. Those projects are far, far greater and bigger and made by way smarter people.

# Install guide
This program works ONLY on Linux. No macOS, no Windows. Maybe you can run it on BSD but that's untested.
For NixOS, you just have to include the derivation in default.nix.
You use NixOS, so you know how to do that.

## 1. Dependencies: rustup, git and figlet
- Arch Linux:
```bash
sudo pacman -S rustup git figlet
```
- Debian Linux:
```bash
sudo apt install rustup git figlet
```

## 2. Clone the repository:
```bash
git clone https://github.com/iVacon/mrfetch
```
## 3. Set up rustup if you haven't already:
```bash
rustup default stable
```
## 4: Compile:
```bash
cd mrfetch
cargo build --release
```
## 5: Install:
```bash
sudo cp target/release/mrfetch /bin/mrfetch
```
