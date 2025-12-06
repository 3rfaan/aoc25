use std::{fs, io};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-02");

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn main() {
    let ranges = parse_input(INPUT_PATH).unwrap(); // Never fails

    let mut invalid_sum = 0;

    for range in ranges {
        check_invalid(&range, &mut invalid_sum);
    }

    dbg!(invalid_sum);
}

fn check_invalid(range: &Range, invalid_sum: &mut u64) {
    for id in range.start..=range.end {
        let digits = count_digits(id);

        let Some((hi, lo)) = split_digits(id, digits) else {
            continue;
        };

        if hi == lo {
            *invalid_sum += id;
        }
    }
}

fn count_digits(n: u64) -> u32 {
    if n < 10 { 1 } else { n.ilog10() + 1 }
}

/// n: integer, d: number of digits
fn split_digits(n: u64, d: u32) -> Option<(u64, u64)> {
    if d % 2 != 0 {
        return None;
    }

    let power = 10u64.pow(d / 2);

    let lo = n % power;
    let hi = n / power;

    Some((hi, lo))
}

fn parse_input(path: &str) -> io::Result<Vec<Range>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .split_terminator(',')
        .filter_map(|range| {
            range.trim().split_once('-').and_then(|(start, end)| {
                Some(Range {
                    start: start.parse().ok()?,
                    end: end.parse().ok()?,
                })
            })
        })
        .collect())
}
