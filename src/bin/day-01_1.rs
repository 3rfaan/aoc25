use std::{fs, io};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-01");

struct Arrow {
    location: i32,
    zero_count: i32,
}

impl Default for Arrow {
    fn default() -> Self {
        Self {
            location: 50,
            zero_count: 0,
        }
    }
}

struct Rotation {
    direction: Direction,
    distance: i32,
}

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let rotations: Vec<Rotation> = parse_input(INPUT_PATH).unwrap(); // Never fails

    let mut arrow = Arrow::default(); // Arrow starts at 50

    for rotation in rotations {
        rotate(&mut arrow, rotation);

        if arrow.location == 0 {
            arrow.zero_count += 1;
        }
    }

    dbg!(arrow.zero_count);
}

fn rotate(arrow: &mut Arrow, rotation: Rotation) {
    const RANGE: i32 = 100;

    match rotation.direction {
        Direction::Right => arrow.location = (arrow.location + rotation.distance) % RANGE,

        Direction::Left => arrow.location = (arrow.location - rotation.distance).rem_euclid(RANGE),
    }
}

fn parse_input(path: &str) -> io::Result<Vec<Rotation>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .lines()
        .filter_map(|line| {
            let mut chars = line.chars();
            let direction: Direction = chars.next()?.into();
            let distance: i32 = chars.as_str().parse().ok()?;

            Some(Rotation {
                direction,
                distance,
            })
        })
        .collect())
}
