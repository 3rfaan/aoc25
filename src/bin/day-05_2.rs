use std::{fs, io};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-05");

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

fn main() {
    let mut ranges = parse_input(INPUT_PATH).unwrap(); // Never fails

    dbg!(count_fresh(&mut ranges));
}

fn count_fresh(ranges: &mut [Range]) -> usize {
    // Sort ranges by start position
    ranges.sort_unstable_by_key(|range| range.start);

    let mut fresh = 0;

    let mut start = ranges[0].start;
    let mut end = ranges[0].end;

    for range in &ranges[1..] {
        if range.start <= end {
            end = end.max(range.end);
        } else {
            fresh += end - start + 1;
            start = range.start;
            end = range.end;
        }
    }

    fresh + (end - start + 1)
}

fn parse_input(path: &str) -> io::Result<Vec<Range>> {
    let input = fs::read_to_string(path)?;

    let (ranges, _) = input.split_once("\n\n").unwrap(); // Never fails

    let ranges: Vec<Range> = ranges
        .lines()
        .filter_map(|line| {
            let (start, end) = line.trim().split_once('-')?;

            Some(Range {
                start: start.parse().ok()?,
                end: end.parse().ok()?,
            })
        })
        .collect();

    Ok(ranges)
}
