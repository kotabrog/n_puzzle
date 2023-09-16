mod frac;
mod generator;
mod reverse_polish_notation;
mod solver;

use anyhow::{Result, anyhow};
use generator::generator;
use reverse_polish_notation::rpn;
use solver::solver;
use frac::Frac;

fn mode_check(args: &Vec<String>) -> Result<String> {
    if args.len() < 2 {
        return Err(anyhow!("Error: Wrong number of arguments"));
    }
    let mode = &args[1];
    if mode != "solve" && mode != "generate" && mode != "check" {
        return Err(anyhow!("Error: Wrong mode"));
    }
    if mode == "solve" {
        if args.len() != 4 {
            return Err(anyhow!("Error: Wrong number of arguments"));
        }
    } else {
        if args.len() != 3 {
            return Err(anyhow!("Error: Wrong number of arguments"));
        }
    }
    Ok(mode.clone())
}

fn solve_args_check(args: &Vec<String>) -> Result<(Vec<u64>, u64)> {
    let mut nums = Vec::new();
    for c in args[2].chars() {
        if !c.is_numeric() {
            return Err(anyhow!("Error: Wrong argument"));
        }
        nums.push(c.to_digit(10).unwrap() as u64);
    }
    Ok((nums, args[3].parse::<u64>().unwrap()))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = mode_check(&args).unwrap();
    if mode == "solve" {
        let (nums, ans) = solve_args_check(&args).unwrap();
        let result = solver(nums, Frac::new(ans as i64, 1).unwrap());
        println!("{:?}", result);
    } else if mode == "check" {
        let frac = rpn(&args[2]).unwrap();
        println!("{}", frac.to_string());
    } else {
        let string = generator(args[2].parse::<usize>().unwrap());
        println!("{}", string);
    }
}
