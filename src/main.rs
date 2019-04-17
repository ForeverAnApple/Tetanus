/**
 * ▄▄▄█████▓▓█████▄▄▄█████▓ ▄▄▄       ███▄    █  █    ██   ██████
 * ▓  ██▒ ▓▒▓█   ▀▓  ██▒ ▓▒▒████▄     ██ ▀█   █  ██  ▓██▒▒██    ▒
 * ▒ ▓██░ ▒░▒███  ▒ ▓██░ ▒░▒██  ▀█▄  ▓██  ▀█ ██▒▓██  ▒██░░ ▓██▄
 * ░ ▓██▓ ░ ▒▓█  ▄░ ▓██▓ ░ ░██▄▄▄▄██ ▓██▒  ▐▌██▒▓▓█  ░██░  ▒   ██▒
 *   ▒██▒ ░ ░▒████▒ ▒██▒ ░  ▓█   ▓██▒▒██░   ▓██░▒▒█████▓ ▒██████▒▒
 *   ▒ ░░   ░░ ▒░ ░ ▒ ░░    ▒▒   ▓▒█░░ ▒░   ▒ ▒ ░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░
 *     ░     ░ ░  ░   ░      ▒   ▒▒ ░░ ░░   ░ ▒░░░▒░ ░ ░ ░ ░▒  ░ ░
 *   ░         ░    ░        ░   ▒      ░   ░ ░  ░░░ ░ ░ ░  ░  ░
 *             ░  ░              ░  ░         ░    ░           ░
 * 
 * Tetanus - A Batch-GCD RSA Cracker.
 *
 * Daiwei Chen, Cole Houston
 * 2019-04-16
 */

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;

//fn analyze(keys: Vec<String>);

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <rsa-file>");
        return Ok(());
    }

    // the ? syntax is like a try catch loop, it's similar to the rust macro try!()
    // Using a BufReader in case of very large files
    let file = File::open(&args[1])?;
    let buf = BufReader::new(file);
    let mut input_keys: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    println!("\nLoaded {} keys from {}.", input_keys.len(), &args[1]);

    analyze(&input_keys);
    
    Ok(())
}

fn analyze(keys: &Vec<String>) {
    println!("Starting analysis on {} keys...", keys.len());
}
