# Rust Project Setup Guide
_______________________________________________________________________________

## Part 1: System Configuration
_______________________________________________________________________________

### Update your path variable

Add this line to your `.zprofile` file. This file is usually located at:
`$HOME/.zprofile`

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

This is where executable binaries from Rust packages 
on `https://crates.io` will be placed.

This allows the shell to access the binaries of globally installed Rust 
packages.

_______________________________________________________________________________

### Restart your computer after updating the path variable

_______________________________________________________________________________

### Install `rustup`, the official toolchain manager for Rust

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

### Initialize the project

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

### Create a `.gitignore` file

```sh
touch .gitignore
```
_______________________________________________________________________________

Add this to the `.gitignore` file
```gitignore
# Build output
target/
```
_______________________________________________________________________________

### Create a `rustfmt.toml` file

```sh
touch rustfmt.toml
```

This file is used to control your settings of `rustfmt`,
which is Rust's built-in code formatter.

Add this to the rustfmt.toml file
```toml
max_width = 80
tab_spaces = 4
trailing_comma = "Never"
```
_______________________________________________________________________________

### Ensure that the project is reproducible

Create a `rust-toolchain.toml` file
```sh
touch rust-toolchain.toml
```
_______________________________________________________________________________

Add this to the file
```toml
[toolchain]
channel = "1.93.1"
components = ["rust-analyzer"]
```

Then build the project
```toml
cargo build
```

When you run the `cargo build` command, 
rustup will have a check the `rust-toolchain.toml` file and ensure that the
project is using the specific channel that was specified in the file 
and that `rust-analyzer` is installed for that specific toolchain. 

_______________________________________________________________________________

### Create a `.cargo` directory

```sh
mkdir .cargo  
```

Create a `config.toml` file and place it in the `.cargo` directory. 

This will allow you to create aliases for cargo commands.

```sh
touch .cargo/config.toml
```

Add this to the file to create an alias called `rq`,
which stands for `run quiet`
```toml
[alias]
rq = "run --quiet"
```
_______________________________________________________________________________

The start of your Rust program is in the file `src/main.rs`

I have replaced the contents of the file with this 
```rust
fn main() {
    println!();
    println!("Rust Project");
    println!();
}
```
_______________________________________________________________________________

To run the code in `src/main.rs`, 
run this from the root of your project directory.
```sh
cargo run
```

You should see something like this
```
   Compiling rust-project v0.1.0 (/home/dezlymacauley/workspace/github-repos/rust-programming-mastery/rust-project)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/rust-project`

Rust Project

```
_______________________________________________________________________________

To run your code so that you only see the output of your program,
you need to add the `--quiet` flag
```sh
cargo run --quiet
```

You should see this now
```

Rust Project

```
_______________________________________________________________________________

And because an alias called `rq` was created earlier in this guide,
you can just run `cargo rq` which is much shorter.

```sh
cargo rq
```

You should get the same output
```

Rust Project

```
_______________________________________________________________________________

Whenever you are not working on your project,
you can delete the target directory to save disk space.

Rust already has a built-in command for this.
```sh
cargo clean
```
_______________________________________________________________________________

To rebuild the project after cleaning it, 
or if you have just cloned the repo from GitHub, 
all you have to do is run `cargo build` or `cargo b` (a built-in Rust alias)

_______________________________________________________________________________
