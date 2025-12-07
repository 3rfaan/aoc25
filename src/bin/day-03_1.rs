use std::{fs, io};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-03");

fn main() {
    let banks = parse_input(INPUT_PATH).unwrap(); // Never fails

    let total: u64 = banks.iter().map(max_joltage).sum();

    dbg!(total);
}

fn max_joltage(bank: &Vec<u8>) -> u64 {
    let mut bank_max = 0;

    for i in 0..bank.len() {
        for j in (i + 1)..bank.len() {
            let val = bank[i] * 10 + bank[j];

            bank_max = bank_max.max(val);
        }
    }

    bank_max as u64
}

fn parse_input(path: &str) -> io::Result<Vec<Vec<u8>>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect())
}
