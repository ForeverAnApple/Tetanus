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

    match args.len() {
        1 => {
            println!("Usage: cargo run <rsa-file>");
            return Ok(());
        },
        _ => { }
    };

    let file = File::open(&args[1])?;
    let buf = BufReader::new(file);
    let mut input_keys: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
    println!("\nLoaded {} keys from {}.", input_keys.len(), &args[1]);
    /*
    for key in input_keys{
        println!("{}", key);
    }
    */

    analyze(&input_keys);
    
    Ok(())
}

fn analyze(keys: &Vec<String>) {
    println!("Starting analysis on {} keys...", keys.len());
}
