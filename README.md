# Tetanus
A Private Key Cracker written in Rust.

# Installation
Tetanus was created to be run in a macOS or Linux environment.

## OpenSSL
For the key recreation module to work, openssl must be installed. 
It can be installed with `sudo apt-get install openssl` for Ubuntu and `sudo pacman -S openssl` for Arch.

## Installing Rust on Linux / MacOS

Just run this:
```
curl https://sh.rustup.rs -sSf | sh
```

## Installing Rust on Windows

Download the windows installer found [here](https://www.rust-lang.org/tools/install), and install with default configuration settings.

## Building and Running the Project

```
cargo run --release help
```
For info on how to run Tetanus.

### Batch GCD on a given set of moduli
Make sure your moduli file contains endline separated hexidecimal moduli. simply run with
```
cargo run --release <moduli file>
```
Tetanus will attempt to find vulnerable moduli and output them into `<moduli file>.vuln` and the GCDs into `<moduli file>.gcd`.

### Benchmarking
Run with
```
cargo run --release benchmark <moduli file>
```
to run a benchmark of [1000, 2500, 5000, 7500, 10000, 15000, 20000] random moduli to perform the Batch-GCD on. There will be 50 trials for each, with outputs of the average time taken (in seconds).

### Test
```
cargo run --release test
```
To run a hardcoded test of Batch-GCD with outputs for product tree, remainder tree, and final gcd process.

### Key Test
```
cargo run keyTest
```
This will create a private key using the modulus and e values found in the public key from the aux_tools/8gwifikeys/ directory. The p value has been taken from the private key in the same directory to make recreation possible without a vulnerable key.

### Key Reconstruction
```
cargo run recreate <moduli file> <line> <optional: e (default=0x10001)>
``` 
The recreate module will take an input file that Batch-GCD has been run on (so that the .vuln and .gcd extensions of that file are in the same directory) and recreate a private key from it. The line number (applies to line number in .vuln file) from which the n and p variables are taken is specified in the next parameter. The e variable is usually 0x10001, but if it is different than this simply specify that difference in the final parameter. If you do not know the e value but think it may not be 0x10001, run `python aux_tools/extract.py <public key>`.
