use std::{fs, io};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input/day-01");

struct Arrow {
    location: i32,
    zero_count: i32,
}

struct Rotation {
    direction: char,
    distance: i32,
}

fn main() {
    let rotations: Vec<Rotation> = parse_input(INPUT_PATH).unwrap(); // Never fails

    let mut arrow = Arrow {
        location: 50,
        zero_count: 0,
    };

    for rotation in rotations {
        rotate(&mut arrow, rotation);
    }

    dbg!(arrow.zero_count);
}

fn rotate(arrow: &mut Arrow, rotation: Rotation) {
    const RANGE: i32 = 100;

    for _ in 0..rotation.distance {
        match rotation.direction {
            'R' => arrow.location = (arrow.location + 1) % RANGE,
            'L' => arrow.location = (arrow.location - 1).rem_euclid(RANGE),
            _ => unreachable!(),
        }

        if arrow.location == 0 {
            arrow.zero_count += 1;
        }
    }
}

fn parse_input(path: &str) -> io::Result<Vec<Rotation>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .lines()
        .filter_map(|line| {
            let mut chars = line.chars();
            let direction = chars.next()?;
            let distance: i32 = chars.as_str().parse().ok()?;

            Some(Rotation {
                direction,
                distance,
            })
        })
        .collect())
}
