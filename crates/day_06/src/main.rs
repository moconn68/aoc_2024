use std::{
    borrow::Borrow,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Result};

const INPUT_FILE: &str = "input.txt";

type GuardMap = Vec<Vec<Space>>;

enum Space {
    Empty,
    Obstacle,
    Guard(Guard),
}

impl<T: Borrow<char>> From<T> for Space {
    fn from(value: T) -> Self {
        match value.borrow() {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            guard => Self::Guard(Guard::from(guard)),
        }
    }
}

#[derive(Clone, Copy)]
enum Guard {
    Up,
    Right,
    Down,
    Left,
}

impl Guard {
    pub const fn rotate(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl<T: Borrow<char>> From<T> for Guard {
    fn from(value: T) -> Self {
        match value.borrow() {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!("Invalid guard character"),
        }
    }
}

fn main() {
    if let Err(e) = runner() {
        eprintln!("ERROR: {}", e);
    }
}

fn runner() -> Result<()> {
    println!("===== DAY 06 =====");

    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(INPUT_FILE);
    let input_reader = BufReader::new(File::open(&input_path)?);
    let input = parse_input(input_reader)?;

    let ans_one = part_one(input)?;
    println!("Part one:\t{}", ans_one);

    Ok(())
}

fn parse_input(raw_input: impl BufRead) -> Result<GuardMap> {
    raw_input
        .lines()
        .map(|line| Ok(line?.chars().map(Space::from).collect()))
        .collect()
}

fn part_one(input: GuardMap) -> Result<usize> {
    let mut visited = HashSet::<(usize, usize)>::new();
    // Find the starting position of the guard
    let (mut pos, mut guard) = input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, space)| match space {
                Space::Guard(guard) => Some(((y, x), *guard)),
                _ => None,
            })
        })
        .context("No guard found in input map")?;
    visited.insert(pos);

    while (pos.0 > 0 && pos.0 < input.len() - 1) && (pos.1 > 0 && pos.1 < input[0].len() - 1) {
        let next_pos = match guard {
            Guard::Up => (pos.0 - 1, pos.1),
            Guard::Right => (pos.0, pos.1 + 1),
            Guard::Down => (pos.0 + 1, pos.1),
            Guard::Left => (pos.0, pos.1 - 1),
        };

        match &input[next_pos.0][next_pos.1] {
            Space::Obstacle => guard.rotate(),
            _ => {
                pos = next_pos;
                visited.insert(pos);
            }
        }
    }

    Ok(visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        let expected = 41;
        let input = parse_input(TEST_INPUT.as_bytes()).unwrap();

        let actual = part_one(input).unwrap();
        assert_eq!(expected, actual);
    }
}
