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
 * 2019-04-26
 * Check LICENSE for licensing information.
 */

use std::fs::File;
use std::io::{self, BufRead, BufReader, LineWriter};
use std::io::prelude::*;
use std::env;
use rug::{Assign, Integer, ops::{Pow, MulFrom, SubFrom, DivFrom, AddFrom, RemFrom}}; // big numbers
use std::time::SystemTime;
use std::cmp::Ordering;
use std::time::Duration;
use rand::Rng;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <rsa-file>");
        return Ok(());
    }

    let mut input_keys: Vec<String> = Vec::new();
    let mut rug_keys: Vec<Integer> = Vec::new();
    
    match args[1].as_ref() {
        "test" => {
            // Input: [1909,2923,291,205,989,62,451,1943,1079,2419]
            // Output should be: [1909, 1, 1, 41, 23, 1, 41, 1, 83, 41]
            input_keys =
                vec!["775".into(), "b6b".into(), "123".into(), "cd".into(), "3dd".into(),
                     "3e".into(), "1c3".into(), "797".into(), "437".into(), "973".into()];
            println!("Testing keys: {:?}", input_keys);

            // Load all the hex keys into rug
            for key in &input_keys{
                let mut parsed = Integer::new();
                parsed.assign(Integer::parse_radix(key, 16).unwrap());
                // println!("Parsing {} into {:?}", key, &parsed);
                rug_keys.push(parsed);
            }

            println!("Running Batch-GCD on {:?}", rug_keys);
            let bgcd = analyze(&rug_keys, true);
            println!("Final GCDs: {:?}", bgcd);

            return Ok(());
        }
        "benchmark" => {
            if args.len() != 3 {
                println!("Usage: cargo run benchmark <moduli file>");
                return Ok(());
            }
            
            println!("Starting Benchmark...");
            let benches = vec![1000, 2500, 5000, 7500, 10000, 15000, 20000];
            let times = benchmark(&benches, &args[2]);
            println!("=============================");
            println!("Results for n and t (in seconds)");
            for (b, t) in benches.iter().zip(&times) {
                println!("n:\t{}\tt:\t{}", b, t);
            }
            
            return Ok(());
        }        
        "recreate" => {
            match args.len(){
                3 => {
                    let n = File::open(&args[2])?;
                    let pq = File::open(&args[3])?;
                    let e = "65537";
                }
                4 => {
                    let n = File::open(&args[2])?;
                    let pq = File::open(&args[3])?;
                    let e = File::open(&args[4])?;    
                }
                _ => {
                    println!("Usage: cargo run recreate <vuln_file> <gcd_file> <optional: e (default is 65537)>");
                    return Ok(());
                }
            }
        }
        "keyTest" => {
            recreate_rsa("11188362659372101913143600898790338707758740049843802890931946943107055234385758698849161576315330979620837055455145878854722494334358352587330793293173497", "11080015406779644339409731862039601570861802205893016182187929091539065741756090484445311749297668806636032643408484361306512552183278449827508244448251953","65537");
            return Ok(());
        }
        _ => {
            // https://github.com/rust-lang/rfcs/pull/2649 Come on rust, you can do it.
            // (input_keys, rug_keys) = load_from_file(&args[1]).unwrap();
            let results = load_from_file(&args[1]).unwrap();
            input_keys = results.0;
            rug_keys = results.1;
        }
    }
    
    //println!("Beginning modulus: {:?}", rug_keys);
    let start = SystemTime::now();
    let bgcd = analyze(&rug_keys, false);
    let time_taken = start.elapsed().unwrap();
    let (vgcds, vulnerable) = output_gcds(&bgcd, &input_keys);
    //println!("Valid GCDS: {:?}\n Vuln Moduli: {:?}", vgcds, vulnerable);
    if vgcds.len() != 0 {
        println!("Found {} Total Vulnerable Moduli", vgcds.len());
        output_files(&vgcds, &vulnerable, &args[1]);
    } else {
        println!("No vulnerable keys found! :(");
    }
    println!("Analysis stage took {} seconds",
             time_taken.as_secs() as f64 + time_taken.subsec_nanos() as f64 * 1e-9);
    
    Ok(())
}

fn load_from_file(fname: &String) -> Result<(Vec<String>, Vec<Integer>), io::Error> {
    let mut input_keys: Vec<String> = Vec::new();
    
    // the ? syntax is like a try catch loop, it's similar to the rust macro try!()
    // Using a BufReader in case of very large files
    let file = File::open(fname)?;
    let buf = BufReader::new(file);
    input_keys = buf.lines().map(|l| l.unwrap()).collect();
    println!("\nLoaded {} keys from {}.", input_keys.len(), fname);
    
    // Load all the hex keys into rug
    let mut rug_keys: Vec<Integer> = Vec::new();
    for key in &input_keys{
        let mut parsed = Integer::new();
        parsed.assign(Integer::parse_radix(key, 16).unwrap());
        // println!("Parsing {} into {:?}", key, &parsed);
        rug_keys.push(parsed);
    }

    Ok((input_keys, rug_keys))
}

// This function takes in a vector of Ns to randomly generate n numbers to benchmark batch-gcd
fn benchmark(Ns: &Vec<i32>, fname: &String) -> Vec<f64> {
    let trials = 50;
    let mut random = rand::thread_rng();
    let mut benches: Vec<f64> = Vec::new();
    let mut rand_nums: Vec<Integer> = Vec::new();
    let (input_keys, rug_keys) = load_from_file(fname).unwrap();

    for n in Ns {
        rand_nums = Vec::new();
        println!("Testing {} moduli", *n);
        for i in 0..*n {
            rand_nums.push(rug_keys[random.gen_range(0, &rug_keys.len())].clone());
        }
        let mut trial_times: Vec<Duration> = Vec::new();
        for i in 0..trials {
            println!("Starting trial {} for n: {}", i, n);
            let start = SystemTime::now();
            let bgcd = analyze(&rand_nums, false);
            let time_taken = start.elapsed().unwrap();
            println!("Trial {} for n: {} took {} seconds", i, n,
                     time_taken.as_secs() as f64 + time_taken.subsec_nanos() as f64 * 1e-9);
            trial_times.push(time_taken);
        }
        let time_sum: f64 = trial_times
            .iter()
            .map(|&x| x.as_secs() as f64 + x.subsec_nanos() as f64 * 1e-9)
            .sum();
        benches.push(time_sum / trials as f64);
    }

    benches
}

fn output_files(gcds: &Vec<Integer>, vulns: &Vec<String>, infile: &String) -> std::io::Result<()> {
    println!("Writing to files...");
    let gcdfilename = infile.clone() + ".gcd";
    let vulfilename = infile.clone() + ".vuln";
    println!("GCD Filename: {}\nVulnerable Moduli Filename: {}", gcdfilename, vulfilename);
    
    let gcdfile = File::create(gcdfilename)?;
    let mut gcdfile = LineWriter::new(gcdfile);
    let vulfile = File::create(vulfilename)?;
    let mut vulfile = LineWriter::new(vulfile);
    
    for (gcd, vuln) in gcds.iter().zip(vulns.iter()) {
        gcdfile.write_all((gcd.to_string_radix(16) + "\n").as_bytes())?;
        vulfile.write_all((vuln.clone() + "\n").as_bytes())?;
    }

    gcdfile.flush()?;
    vulfile.flush()?;
    println!("All done! Phew!");
    Ok(())
}

fn output_gcds(gcds: &Vec<Integer>, moduli: &Vec<String>) -> (Vec<Integer>, Vec<String>){
    let mut valid_gcds: Vec<Integer> = Vec::new();
    let mut vuln: Vec<String> = Vec::new();
    for (i, gcd) in gcds.iter().enumerate() {
        if gcd.cmp(&Integer::from(1)) != Ordering::Equal {
            valid_gcds.push(Integer::from(gcd));
            vuln.push((&moduli[i]).clone());
        }
    }

    (valid_gcds, vuln)
}

fn analyze(keys: &Vec<Integer>, test: bool) -> Vec<Integer>{
    println!("Starting analysis on {} keys...", keys.len());
    let prod_tree = product_tree(&keys);
    let rem_tree = remainder_tree(&prod_tree, &keys);
    if test {
        println!("Generated producted tree: {:#?}", prod_tree);
        println!("Generated remainder tree: {:?}", rem_tree);
    }
    batch_gcd(&rem_tree, &keys)

}

// Using a product tree here will speed up the Batch-GCD significantly. O(lg n) instead of O(n).
fn product_tree(keys: &Vec<Integer>) -> Vec<Vec<Integer>> {
    let mut prods: Vec<Vec<Integer>> = Vec::new();
    let mut leaf_layer = keys.to_vec();
    prods.push(leaf_layer.to_vec());
    while leaf_layer.len() > 1 {
        let mut temp_layer = Vec::new();
        for i in 0..((leaf_layer.len())/2) {
            // Using a buffer here due to rug memory allocation optimizations and issues with floats
            // and more complex numbers
            let mut prod_buf = Integer::new();
            prod_buf.assign(&leaf_layer[i*2] * &leaf_layer[i*2+1]);
            // println!("Sig bits: {}", prod_buf.significant_bits());
            temp_layer.push(prod_buf);    
        }
        
        if leaf_layer.len() % 2 == 1{
            //let mut last = Integer::new();
            //last.assign(&leaf_layer[&leaf_layer.len()-1]);
            temp_layer.push(leaf_layer.pop().unwrap());
        }
        
        leaf_layer = temp_layer.to_vec();
        //println!("{:?}", leaf_layer);
        prods.push(leaf_layer.to_vec());
    }
    
    prods
}

fn remainder_tree(prod_tree: &Vec<Vec<Integer>>, keys: &Vec<Integer>) -> Vec<Integer> {
    let mut temp_ptree = prod_tree.to_vec();
    let mut rems: Vec<Integer> = vec![Integer::new(); keys.len()];
    let rootnum = temp_ptree.pop().unwrap().pop().unwrap();
    
    //println!("Size of rem: {}", rems.len());
    rems[0] = rootnum;
    for prod in temp_ptree.iter().rev(){        
        //println!("prod: {:?}", prod);
        for i in (0..prod.len()).rev() {
            //println!("Rems at {}: {:?}", i/2, rems);
            let mut ppow = Integer::new();
            ppow.assign((&prod[i]).pow(2));
            let incomplete = &rems[i/2] % ppow;
            &rems[i].assign(incomplete);
            //println!("{} % {} ** 2 = {}", &rems[i/2], &prod[i], &rems[i]);
        }
    }
    /*
    for key in keys {
    let mut modu = Integer::new();
    modu.assign(&rootnum % key);
    rems.push(modu);
}
     */
    rems
}

fn batch_gcd(rem_tree: &Vec<Integer>, keys: &Vec<Integer>) -> Vec<Integer> {
    let mut bgcd: Vec<Integer> = Vec::new();

    for i in 0..keys.len() {
        let mut div = Integer::new();
        div.assign(&rem_tree[i] / &keys[i]);
        let gcd = div.gcd(&keys[i]);
        bgcd.push(gcd);
    }
    
    bgcd
}

fn recreate_rsa(mut stringP:&str, mut stringQ:&str, mut encryption:&str) {
    println!("asn1=SEQUENCE:private_key");
    println!("[private_key]");
    println!("version=INTEGER:0");

	  let mut n = Integer::new();
	  let mut p = Integer::new();
	  let mut q = Integer::new();
	  let mut e = Integer::new();

	  n.assign(Integer::parse(stringP).unwrap());
	  p.assign(Integer::parse(stringP).unwrap());
	  q.assign(Integer::parse(stringQ).unwrap());
	  e.assign(Integer::parse(encryption).unwrap());

  	let mut phi=Integer::from(1);
  	let mut p2=Integer::from(1);
  	let mut q2=Integer::from(1);

  	n.mul_from(&q);
 	  e.add_from(0);

 	  println!("n=INTEGER:0x{:x}", n);
  	println!("e=INTEGER:0x{:x}", e);

  	phi.sub_from(&p);
  	p2.sub_from(&p);
  	q2.sub_from(&q);
 	  phi.mul_from(&q2);
	  let mut d=Integer::from(e);
	  d.invert_mut(&phi);

	  println!("d=INTEGER:0x{:x}", d);

	  println!("p=INTEGER:0x{:x}", p);
  	println!("q=INTEGER:0x{:x}", q);
  	
  	p2.rem_from(&d);
  	q2.rem_from(&d);
  	println!("exp1=INTEGER:0x{:x}", p2);
  	println!("exp2=INTEGER:0x{:x}", q2);

  	let expo = Integer::from(-1);
  	let power = q.pow_mod(&expo, &p).unwrap();
  	println!("coeff=INTEGER:0x{:x}", power);
}
