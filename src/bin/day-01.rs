use std::fs;

const INPUT_PATH: &str = "../../input/day-01";

struct Arrow {
    location: u16,
    zero_count: u16,
}

impl Arrow {
    fn new() -> Self {
        Self {
            location: 50,
            zero_count: 0,
        }
    }
}

struct Rotation {
    direction: Direction,
    distance: u16,
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

fn main() -> std::io::Result<()> {
    let rotations: Vec<Rotation> = parse_input(INPUT_PATH)?;

    let mut arrow = Arrow::new();

    for rotation in rotations {
        rotate(&mut arrow.location, rotation);

        if arrow.location == 0 {
            arrow.zero_count += 1;
        }
    }

    dbg!(arrow.zero_count);
    Ok(())
}

fn rotate(arrow_location: &mut u16, rotation: Rotation) {
    const RANGE: u16 = 100;

    match rotation.direction {
        Direction::Left => {
            *arrow_location = (*arrow_location + RANGE - (rotation.distance % RANGE)) % RANGE
        }

        Direction::Right => *arrow_location = (*arrow_location + rotation.distance) % RANGE,
    }
}

fn parse_input(path: &str) -> std::io::Result<Vec<Rotation>> {
    let input = fs::read_to_string(path)?;

    Ok(input
        .lines()
        .filter_map(|line| {
            let mut chars = line.chars();
            let direction: Direction = chars.next()?.into();
            let rotation: u16 = chars.as_str().parse().ok()?;

            Some(Rotation {
                direction,
                distance: rotation,
            })
        })
        .collect())
}
