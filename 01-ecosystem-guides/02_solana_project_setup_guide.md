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
export PATH="$HOME/.local/bin/surfpool-cli:$PATH"
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
solana-release-x86_64-unknown-linux-gnu.tar.bz2
```

There should be a hash next to it that looks like this:
```
sha256:8cffaecc3a5f3c47c8d8f94ea739c716fbae08683018fc29b65c3045a0e9bdd2
```

Run `sha256sum` on the file to check if the commit hash matches the 
one on GitHub:
```sh

sha256sum solana-release-x86_64-unknown-linux-gnu.tar.bz2
```

To extract the file:
```sh
tar -xjf solana-release-x86_64-unknown-linux-gnu.tar.bz2
```

After extracting the file you should see this directory:
```
solana-release
```

Move the binaries in `solana-release` directory to `$HOME/.local/solana-cli/`
```sh
mkdir -p $HOME/.local/bin/solana-cli
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

You can get the binaries here
```
https://github.com/solana-foundation/surfpool/releases
```

Look for something like this:
```
v1.0.1 Latest

chore(release): publish v1.0.1
```

Download this file
```sh
surfpool-linux-x64.tar.gz
```

Extract the file:
```sh
tar -xzf surfpool-linux-x64.tar.gz
```

It is a single file, so just move it to the correct location:
```sh
mkdir ~/.local/bin/surfpool-cli
cp surfpool ~/.local/bin/surfpool-cli/
chmod +x ~/.local/bin/surfpool-cli/surfpool
```
_______________________________________________________________________________

### To confirm that everything is working, run this command

Rust
```sh
rustc --version
cargo --version
```

Node.js and Yarn
```sh
node --version
yarn --version
```

Solana CLI, Anchor CLI, Surfpool Clis
```sh
solana --version
anchor --version
surfpool --version
```
_______________________________________________________________________________

To get your Solana config:
```sh
solana config get
```
_______________________________________________________________________________

To use a localhost:
```sh
solana config set -u l
```
_______________________________________________________________________________

To use devnet (This is what I'll be using)
```sh
solana config set -u d
```
_______________________________________________________________________________

To create a wallet
```sh
solana-keygen new
```
_______________________________________________________________________________

To check your address (public key)
```sh
solana address
```
_______________________________________________________________________________

### How to get test SOL (Fake currency for testing)

First make sure that the Solana CLI is set to the correct network 
where you are deploying your project to:

E.g. I want to deploy to `devnet`

```sh
solana config set -u d
```

Now I want to request 1 SOL test SOL
```sh
solana airdrop 1
```
_______________________________________________________________________________

If you have issues with the CLI, you can use the Solana Faucett:

```
https://faucet.solana.com/
```
_______________________________________________________________________________

To check your wallet balance:
```sh
solana config set -u d
solana balance
```
_______________________________________________________________________________

How to run a local validator (A loca Solana blockchain)

Open a seperate channel and run this command.

```sh
solana config set -u l
solana-test-validator
```
_______________________________________________________________________________
