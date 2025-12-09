use std::{fs, io};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-05");

#[derive(Debug)]
struct Database {
    ranges: Vec<Range>,
    ids: Vec<usize>,
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, value: usize) -> bool {
        self.start <= value && value <= self.end
    }
}

fn main() {
    let database = parse_input(INPUT_PATH).unwrap(); // Never fails

    dbg!(count_fresh(&database));
}

fn count_fresh(database: &Database) -> usize {
    database
        .ids
        .iter()
        .filter(|&id| database.ranges.iter().any(|range| range.contains(*id)))
        .count()
}

fn parse_input(path: &str) -> io::Result<Database> {
    let input = fs::read_to_string(path)?;

    let (ranges, ids) = input.split_once("\n\n").unwrap(); // Never fails

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

    let ids: Vec<usize> = ids
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();

    Ok(Database { ranges, ids })
}
