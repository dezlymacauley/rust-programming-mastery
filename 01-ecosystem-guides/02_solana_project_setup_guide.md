# Solana Project Setup Guide
_______________________________________________________________________________
## Part 1: System Configuration
_______________________________________________________________________________

### Update your path variable

Add these lines to your `.zprofile` file. This file is usually located at:
`$HOME/.zprofile`

```sh
export PATH="$HOME/.local/bin/solana-cli:$PATH"
export PATH="$HOME/.local/bin/anchor-cli:$PATH"
```

This is where the executable binaries for the `Solana Cli`, `Anchor Cli`,
and `Surfpool Cli` will be saved.
_______________________________________________________________________________

## Restart your computer after updating your path

_______________________________________________________________________________

### Install `Rust` 
_______________________________________________________________________________

### Install `Node.js` and `Yarn`

The most widely used JavaScript runtime (Required by the Anchor framework)
```sh
sudo pacman -S --needed nodejs
```

A JavaScript package manager required by the Anchor smart contract framework
```sh
sudo pacman -S --needed yarn
```
_______________________________________________________________________________

### Install `Solana Cli` 

The Solana CLI provides all the tools required to build 
and deploy Solana programs.

Download the pre-built binary here
```
https://github.com/anza-xyz/agave/releases
```

Look for something like this:
```
Release v3.1.9 Latest

This a stable release suitable for Testnet, Devnet and Mainnet Beta.
```

Scroll down to `Assets` and download this:
```
solana-release-x86_64-apple-darwin.tar.bz2 
```

There should be a hash next to it that looks like this:
```
sha256:db27a2743edabb4dcff204993b70ebec79f471a526ae88be90e1d8e2644d87e2
```

Run `sha256sum` on the file to check if the commit hash matches the 
one on GitHub:
```sh
sha256sum -xjf solana-release-x86_64-apple-darwin.tar.bz2
```

To extract the file:
```sh
tar -xjf solana-release-x86_64-apple-darwin.tar.bz2
```

After extracting the file you should see this directory:
```
solana-release
```

Move the `solana-release` directory to `$HOME/.local/bin`
```sh
mkdir $HOME/.local/bin/solana-cli
find solana-release/bin -maxdepth 1 -type f -exec cp {} ~/.local/bin/solana-cli/ \;
```
_______________________________________________________________________________

### Install `Anchor` 

Anchor is a framework for developing Solana programs. 

The Anchor framework leverages Rust macros 
to simplify the process of writing Solana programs.

Note: Anchor requires `Node.js` and `Yarn` in order to use its default
TypeScript project setup later in this guide.

Download the pre-built binary here
```
https://github.com/coral-xyz/anchor/releases
```

Look for the latest release. E.g.
```
v0.32.1 Latest
```

Download this this file:
```
anchor-0.32.1-x86_64-unknown-linux-gnu 
```

It is a single binary file so the only thing that you need to do is this:
```sh
mkdir $HOME/.local/bin/anchor-cli
cp anchor-0.32.1-x86_64-unknown-linux-gnu $HOME/.local/bin/anchor-cli/anchor
chmod +x $HOME/.local/bin/anchor-cli/anchor
```
_______________________________________________________________________________

### Install `Surfpool Cli` 

Surfpool is a tool for local development and 
an improved replacement for solana-test-validator. 

Learn more about Surfpool features in the Surfpool documentation.

_______________________________________________________________________________

