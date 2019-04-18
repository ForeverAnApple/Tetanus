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
use rug::{Assign, Integer, ops::Pow}; // big numbers

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <rsa-file>");
        return Ok(());
    }

    let mut input_keys: Vec<String> = Vec::new();
    
    match args[1].as_ref() {
        "test" => {
            // Input: [1909,2923,291,205,989,62,451,1943,1079,2419]
            // Output should be: [1909, 1, 1, 41, 23, 1, 41, 1, 83, 41]
            input_keys =
                vec!["775".into(), "b6b".into(), "123".into(), "cd".into(), "3dd".into(),
                     "3e".into(), "1c3".into(), "797".into(), "437".into(), "973".into()];
            println!("Testing keys: {:?}", input_keys);
        }
        "benchmark" => {
            println!("Starting Benchmark...");
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
    for key in input_keys{
        let mut parsed = Integer::new();
        parsed.assign(Integer::parse_radix(key, 16).unwrap());
        // println!("Parsing {} into {:?}", key, &parsed);
        rug_keys.push(parsed);
    }
    
    analyze(&rug_keys);
    
    Ok(())
}

fn analyze(keys: &Vec<Integer>) {
    println!("Starting analysis on {} keys...", keys.len());
    let prod_tree = product_tree(&keys);
    //println!("Generated producted tree: {:#?}", prod_tree);
}

// Using a product tree here will speed up the Batch-GCD significantly. O(lg n) instead of O(n).
fn product_tree(keys: &Vec<Integer>) -> Vec<Vec<Integer>> {
    let mut prods: Vec<Vec<Integer>> = Vec::new();
    let mut leaf_layer = keys.to_vec();
    prods.push(leaf_layer.to_vec());
    
    while leaf_layer.len() > 1 {
        let mut temp_layer = Vec::new();
        for i in 0..((leaf_layer.len()+1)/2) {
            // Using a buffer here due to rug memory allocation optimizations and issues with floats
            // and more complex numbers
            let mut prod_buf = Integer::new();
            let incomplete = &leaf_layer[i] * &leaf_layer[i+1];
            prod_buf.assign(incomplete);
            println!("Sig bits: {}", prod_buf.significant_bits());
            temp_layer.push(prod_buf);
        }
        leaf_layer = temp_layer.to_vec();
        prods.push(leaf_layer.to_vec());
    }
    
    prods
}
