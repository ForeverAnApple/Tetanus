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
 * Check LICENSE for licensing information.
 */

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;
use rug::{Assign, Integer}; // big numbers

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <rsa-file>");
        return Ok(());
    }

    let mut input_keys: Vec<String> = Vec::new();
    
    match args[1].as_ref() {
        "test" => {
            input_keys =
                vec!["775".into(), "b6b".into(), "123".into(), "cd".into(), "3dd".into(),
                     "3e".into(), "1c3".into(), "797".into(), "437".into(), "973".into()];
            println!("Testing keys: {:?}", input_keys);
        }
        _ => {
            // the ? syntax is like a try catch loop, it's similar to the rust macro try!()
            // Using a BufReader in case of very large files
            let file = File::open(&args[1])?;
            let buf = BufReader::new(file);
            input_keys = buf.lines().map(|l| l.unwrap()).collect();
            println!("\nLoaded {} keys from {}.", input_keys.len(), &args[1]);
        }
    }
    
    
    // Load all the hex keys into rug
    let mut rug_keys: Vec<Integer> = Vec::new();
    for (i, key) in input_keys.iter().enumerate(){
        let mut parsed = Integer::new();
        parsed.assign(Integer::parse_radix(key, 16).unwrap());
        println!("Parsing {} into {:?}", key, &parsed);
        rug_keys.push(parsed);
    }
    
    analyze(&rug_keys);
    
    Ok(())
}

fn analyze(keys: &Vec<Integer>) {
    println!("Starting analysis on {} keys...", keys.len());
    
}

fn product() {

}
