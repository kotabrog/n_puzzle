mod frac;
mod generator;
mod reverse_polish_notation;

use anyhow::{Result, anyhow};
use generator::generator;
use reverse_polish_notation::rpn;

fn mode_check(args: &Vec<String>) -> Result<String> {
    if args.len() < 2 {
        return Err(anyhow!("Error: Wrong number of arguments"));
    }
    let mode = &args[1];
    if mode != "solve" && mode != "generate" && mode != "check" {
        return Err(anyhow!("Error: Wrong mode"));
    }
    if args.len() != 3 {
        return Err(anyhow!("Error: Wrong number of arguments"));
    }
    Ok(mode.clone())
}

fn solve_args_check(args: &Vec<String>) -> Result<Vec<u32>> {
    let mut nums = Vec::new();
    for c in args[2].chars() {
        if !c.is_numeric() {
            return Err(anyhow!("Error: Wrong argument"));
        }
        nums.push(c.to_digit(10).unwrap());
    }
    Ok(nums)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = mode_check(&args).unwrap();
    if mode == "solve" {
        let nums = solve_args_check(&args).unwrap();
        println!("{:?}", nums);
    } else if mode == "check" {
        let frac = rpn(&args[2]).unwrap();
        println!("{}", frac.to_string());
    } else {
        let string = generator(args[2].parse::<usize>().unwrap());
        println!("{}", string);
    }
}
