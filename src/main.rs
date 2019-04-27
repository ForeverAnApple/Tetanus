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
use std::io::{self, BufRead, BufReader, LineWriter, Write, Error};
use std::io::prelude::*;
use std::env;
use std::process::Command;
use rug::{Assign, Integer, ops::{Pow, MulFrom, SubFrom, AddFrom, RemFrom, DivFrom}}; // big numbers
use std::time::SystemTime;
use std::cmp::Ordering;
use std::time::Duration;
use rand::Rng;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <rsa-file>\n");
        println!("For more functionality: cargo run help");
        return Ok(());
    }

    let mut input_keys: Vec<String> = Vec::new();
    let mut rug_keys: Vec<Integer> = Vec::new();
    
    match args[1].as_ref() {
        "help" => {
            println!("Tetanus uses Batch-GCD to find vulnerable RSA keys in a given pool. Once vulnerable keys are found, Tetanus can reconstruct the private key.\n");
            println!("cargo run <file>: will run batch-GCD on given file of moduli\n");
            println!("cargo run recreate <file> <line>: recreates private key from given line of a file\n\t\t\t\t\t(using the line number from file.vuln)");
            println!("cargo run help: brings up this screen\n");
            println!("cargo run benchmark <file>: times 50 trials of Batch-GCD on given file\n");
            println!("cargo run test: proves Batch-GCD supplies correct values\n");
            println!("cargo run keyTest: proves RSA private key is successfully reconstructed\n");
            return Ok(());
        }
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
            println!("You must have openssl installed for this module to run successfully");
            match args.len(){
                4 => {
                    // sets iterator, line number, and e variables
                    let i:i32 = 1;
                    let line = &args[3].parse::<i32>().unwrap();
                    let e = "10001";
                    println!("Creating RSA private key from line {} and e {}", line, e);

                    // opens .vuln file to get n
                    let n = File::open((&args[2] as &str).to_owned()+".vuln")?;
                    let mut buf = String::new();
                    let mut nbuf = BufReader::new(n);
                    nbuf.read_line(&mut buf);
                    // moves buffer to line specified by user
                    for i in 0..*line {
                        buf.clear();
                        nbuf.read_line(&mut buf);
                    }
                    buf.pop();
                    // creates variable of type &str that references number in buffer
                    let mut strN:&str = &buf;

                    // does the same as above but for the .gcd file to get p
                    let p = File::open((&args[2] as &str).to_owned()+".gcd")?;
                    let mut buf2 = String::new();
                    let mut pbuf = BufReader::new(p);
                    pbuf.read_line(&mut buf2);
                    for i in 0..*line {
                        buf2.clear();
                        pbuf.read_line(&mut buf2);
                    }
                    buf2.pop();
                    let mut strP:&str = &buf2;

                    // runs recreate_rsa on obtained n and p with an e of 0x10001
                    recreate_rsa(&strN, &strP, e);

                    return Ok(());
                }
                5 => {
                    // sets iterator, line number, and e variables
                    let i:i32 = 1;
                    let line = &args[3].parse::<u32>().unwrap();
                    let e = &args[4];
                    println!("Creating RSA private key from line {} and e {}", line, e);

                    // opens .vuln file to get n
                    let n = File::open((&args[2] as &str).to_owned()+".vuln")?;
                    let mut buf = String::new();
                    let mut nbuf = BufReader::new(n);
                    nbuf.read_line(&mut buf);
                    // moves buffer to line specified by user
                    for i in 0..*line {
                        buf.clear();
                        nbuf.read_line(&mut buf);
                    }
                    buf.pop();
                    // creates variable of type &str that references number in buffer
                    let mut strN:&str = &buf;

                    // does the same as above but for the .gcd file to get p
                    let p = File::open((&args[2] as &str).to_owned()+".gcd")?;
                    let mut buf2 = String::new();
                    let mut pbuf = BufReader::new(p);
                    pbuf.read_line(&mut buf2);
                    for i in 0..*line {
                        buf2.clear();
                        pbuf.read_line(&mut buf2);
                    }
                    buf2.pop();
                    let mut strP:&str = &buf2;

                    // runs recreate_rsa on obtained n and p and user specified e
                    recreate_rsa(&strN, &strP, e);
                      
                    return Ok(());
                }
                _ => {
                    println!("Usage: cargo run recreate <file> <line from file> <optional: e (default is 0x10001)>");
                    println!("(file refers to the file that has already been run through tetanus)", );
                    return Ok(());
                }
            }
        }

        "keyTest" => {
            println!("You must have openssl installed for this module to run successfully");
            println!("Testing key regen with hardcoded values...");
            recreate_rsa("b089029fe0b4b5b785fef2bbae1d650d7ffc72cde478739174a589f660b3092f2a2f943afe593bd0be5165c947aa5769e6180b1e5bd7ed5ae6471621d9a7c321a266b591de3dddb080359025933b9a9dd8e5ddb38ee5c6dbb12dba59ae8fd36eab9376be2a2bcd78809706a7abda3e915d61c3d313ee8d4e84fd5cc73e25f1a9", "d59f93d6e5a6ce24d0d463666dc2bfd5be5d214ef4da29a40c15ffdf92030dc4c6599288a1b86ed64e90aaf6aae2310e7067cd6dbf35cac41ab980ee5f2352f9","10001");
            println!("This key can be tested for validity with the keypair found in ./aux_tools/8gwifikeys");
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

fn recreate_rsa(mut stringN:&str, mut stringP:&str, mut encryption:&str) {
    let mut f = File::create("recreation.txt").expect("Error: Unable to create file");
    println!("Creating file recreation.txt to write key values to");

    // writes first 3 lines to recreation file
    f.write_all("asn1=SEQUENCE:private_key\n".as_bytes()).expect("Error: Unable to write data");
    f.write_all("[private_key]\n".as_bytes()).expect("Error: Unable to write data");
    f.write_all("version=INTEGER:0\n".as_bytes()).expect("Error: Unable to write data");

    // creates new variables of type Integer
    let mut n = Integer::new();
    let mut p = Integer::new();
    let mut q = Integer::new();
    let mut e = Integer::new();

    // assigns values to Integers
    n.assign(Integer::parse_radix(stringN, 16).unwrap());
    p.assign(Integer::parse_radix(stringP, 16).unwrap());
    q.assign(Integer::parse_radix(stringP, 16).unwrap());
    e.assign(Integer::parse_radix(encryption, 16).unwrap());

    // creates 3 Integers containing 1 so they can be used to subtract one from above Integers later
  	let mut phi=Integer::from(1);
  	let mut p2=Integer::from(1);
  	let mut q2=Integer::from(1);

    // q = n/q
  	q.div_from(&n);
 	//e.add_from(0);

    let strN = "n=INTEGER:0x".to_owned()+&n.to_string_radix(16);
    f.write_all(strN.as_bytes()).expect("Error: Unable to write data");

    let strE = "\ne=INTEGER:0x".to_owned()+&e.to_string_radix(16);
    f.write_all(strE.as_bytes()).expect("Error: Unable to write data");

    // p2 and q2 are p-1 and q-1.  phi is (p-1)*(q-1)
  	phi.sub_from(&p);
  	p2.sub_from(&p);
  	q2.sub_from(&q);
    phi.mul_from(&q2);
    let mut d=Integer::from(e);
    // d is e invert mod phi
    d.invert_mut(&phi);

    let strD = "\nd=INTEGER:0x".to_owned()+&d.to_string_radix(16);
    f.write_all(strD.as_bytes()).expect("Error: Unable to write data");

    let strP = "\np=INTEGER:0x".to_owned()+&p.to_string_radix(16);
    f.write_all(strP.as_bytes()).expect("Error: Unable to write data");
    let strQ = "\nq=INTEGER:0x".to_owned()+&q.to_string_radix(16);
    f.write_all(strQ.as_bytes()).expect("Error: Unable to write data");
  	
    // d mod p2 and d mod q2 to create exponent 1 and 2
  	p2.rem_from(&d);
  	q2.rem_from(&d);
    let strP2 = "\nexp1=INTEGER:0x".to_owned()+&p2.to_string_radix(16);
    f.write_all(strP2.as_bytes()).expect("Error: Unable to write data");
    let strQ2 = "\nexp2=INTEGER:0x".to_owned()+&q2.to_string_radix(16);
    f.write_all(strQ2.as_bytes()).expect("Error: Unable to write data");

    // the coefficient is (inverse q) mod p
  	let expo = Integer::from(-1);
  	let power = q.pow_mod(&expo, &p).unwrap();
    let strCoeff = "\ncoeff=INTEGER:0x".to_owned()+&power.to_string_radix(16);
    f.write_all(strCoeff.as_bytes()).expect("Error: Unable to write data");
    f.write_all("\n".as_bytes()).expect("Error: Unable to write data");

    // runs bash script in aux_tools to reconstruct private key
    println!("Running auxiliary bash script on key values to reconstruct key");
    let mut cmd = Command::new("bash");
    cmd.arg("aux_tools/keyRegen.sh");
    cmd.arg("recreation.txt");
    cmd.arg("rsa.key");
    match cmd.output() {
        Ok(o) => {
            unsafe{
                println!("{}", String::from_utf8_unchecked(o.stdout));
                println!("The RSA private key has been saved to rsa.key");
            }
        }
        Err(e) => {
            println!("Error");
        }
    }
    // disclaimer: p and q could be mixed up
    println!("\nNote: there is a change the p and q values were mixed up.");
    println!("If the private key does not seem to work, try dividing the number in");
    println!("moduli.vuln by the number on the corresponding line of moduli.gcd.");
    println!("Replace the line in moduli.gcd with the result (in hex) and run again.");
}
