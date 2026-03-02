# Rust Project Setup Guide
_______________________________________________________________________________

## Part 1: System Configuration
_______________________________________________________________________________

Install `rustup`. This is the official toolchain manager for Rust.

```sh
sudo pacman -S --needed rustup
```
_______________________________________________________________________________

Use `rustup` to set the default the toolchain on your system to the toolchain 
called `stable`. This toolchain will ensure that your system is using 
the latest stable version of the Rust toolchain.

A toolchain is a collection of tools that include:
- `rustc` (the Rust compiler), 
- `cargo` (the rust package manager)
- `rustfmt` (a formatter for Rust)
- and many more tools...

Since you don't have the `stable` toolchain installed,
`rustup` will automatically download it.

```sh
rustup default stable
```
_______________________________________________________________________________

Rust toolchains do not include language support for Rust files by default.

To get language support for Rust when using the `stable` toolchain,
you need to install `rust-analyzer` for that specific toolchain.

```sh
rustup component add \
--toolchain stable \
rust-analyzer
```
_______________________________________________________________________________

## Part 2: Project Configuration

Create the project directory
```sh
mkdir rust-project
```

Enter the project directory
```sh
cd rust-project
```

For the rest of this guide, all commands must be run from this directory.

_______________________________________________________________________________

Initialize the project
```sh
cargo init --vcs none
```

The flag `--vcs none` will ensure that no version control system is setup 
in the project. I prefer to setup the `git` version control system myself.

_______________________________________________________________________________

You should now have this structure:

```
.
├── Cargo.toml
└── src
    └── main.rs
```
_______________________________________________________________________________

Create a `.gitignore` file

```sh
touch .gitignore
```
_______________________________________________________________________________

Add this to the `.gitignore` file
```gitignore
# Build output
/target/
```
_______________________________________________________________________________
