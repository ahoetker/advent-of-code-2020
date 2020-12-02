use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_input() -> std::io::Result<Vec<u32>> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    Ok(reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect())
}

fn main() -> std::io::Result<()> {
    let nums = read_input()?;

    // Part 1
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == 2020 {
                println!("{}", nums[i] * nums[j]); 
            }
        }
    }

    // Part 2
    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!("{}", nums[i] * nums[j] * nums[k]);
                }
            }
        }
    }
    Ok(())
}
