use std::fs;
use std::io;

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-04");

struct Diagram {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<u8>>,
}

#[rustfmt::skip]
const DIRS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1), /*----*/ ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn main() {
    let mut diagram = parse_input(INPUT_PATH).unwrap(); // Never fails

    dbg!(count_removable(&mut diagram));
}

fn count_removable(diagram: &mut Diagram) -> usize {
    let mut removed = 0;
    let mut batch = Vec::with_capacity(diagram.rows * diagram.cols / 4);

    loop {
        // Loop through grid, check if paper roll is accessible and push it to batch
        (0..diagram.rows)
            .flat_map(|row| (0..diagram.cols).map(move |col| (row, col)))
            .filter(|&(row, col)| is_accessible(diagram, row, col))
            .for_each(|(row, col)| batch.push((row, col)));

        // If batch is empty there are no more removable paper rolls
        if batch.is_empty() {
            break;
        }

        // Remove paper roll (replace @ with .)
        for &(row, col) in &batch {
            diagram.grid[row][col] = b'.';
        }

        // Add amount of removed paper rolls to counter
        removed += batch.len();

        // Clear batch for next iteration
        batch.clear();
    }

    removed
}

fn is_accessible(diagram: &Diagram, row: usize, col: usize) -> bool {
    if diagram.grid[row][col] != b'@' {
        return false;
    }

    let adjacent = DIRS
        .iter()
        .filter(|(dx, dy)| {
            let r = (row as isize + dx) as usize;
            let c = (col as isize + dy) as usize;

            r < diagram.rows && c < diagram.cols && diagram.grid[r][c] == b'@'
        })
        .count();

    adjacent < 4
}

fn parse_input(path: &str) -> io::Result<Diagram> {
    let input = fs::read_to_string(path)?;

    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let rows = grid.len();
    let cols = grid.first().map_or(0, |row| row.len());

    Ok(Diagram { rows, cols, grid })
}
