# Tetanus
A Private Key Cracker written in Rust.

# Installation
Tetanus was created to be run in a macOS or Linux environment.

To setup rust easily in a macOS or Linux environment, run the following command: 
`curl https://sh.rustup.rs -sSf | sh`

For the key recreation module to work, openssl must be installed. 
It can be installed with `sudo apt-get install openssl`

Once you have cloned this repository, cd to Tetanus and run `cargo run` to start the program.
`cargo run --release` may also be used to spend extra time compiling a faster version.

The command `cargo run help` will briefly explain each module of the program.

TL;DR
`curl https://sh.rustup.rs -sSf | sh`
`sudo apt-get install openssl -y`
`git clone https://github.com/ForeverAnApple/Tetanus.git`
`cd Tetanus`
`cargo run --release`
`cargo run help`
