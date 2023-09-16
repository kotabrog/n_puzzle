mod frac;
mod generator;

use anyhow::{Result, anyhow};
use generator::generator;

fn mode_check(args: &Vec<String>) -> Result<String> {
    if args.len() < 2 {
        return Err(anyhow!("Error: Wrong number of arguments"));
    }
    let mode = &args[1];
    if mode != "solve" && mode != "generate" {
        return Err(anyhow!("Error: Wrong mode"));
    }
    if mode == "generate" && args.len() != 3 {
        return Err(anyhow!("Error: Wrong number of arguments"));
    }
    Ok(mode.clone())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = mode_check(&args).unwrap();
    if mode == "solve" {
        println!("Solve");
    } else {
        let string = generator(args[2].parse::<usize>().unwrap());
        println!("{}", string);
    }
}
