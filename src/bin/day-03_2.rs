use std::{fs, io, mem};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-03");

const AMOUNT: usize = 12; // Amount of batteries to take from each bank

fn main() {
    let mut banks = parse_input(INPUT_PATH).unwrap(); // Never fails

    let total: u64 = banks.iter_mut().map(max_joltage).sum();

    dbg!(total);
}

fn max_joltage(bank: &mut Vec<u8>) -> u64 {
    // Batteries with largest joltage
    let mut batteries = [0; AMOUNT];

    // Copy last 12 elements into an array
    let end = bank.len() - AMOUNT;
    batteries.copy_from_slice(&bank[end..]);

    // Process remaining batteries
    for next in bank[..end].iter_mut().rev() {
        for battery in batteries.iter_mut() {
            if next < battery {
                break;
            }
            mem::swap(battery, next);
        }
    }

    batteries
        .iter()
        .fold(0, |joltage, &battery| 10 * joltage + battery as u64)
}

fn parse_input(path: &str) -> io::Result<Vec<Vec<u8>>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect())
}
