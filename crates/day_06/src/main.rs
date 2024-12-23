use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
enum Guard {
    #[default]
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

    /// Given the guard's current position and orientation, determine the next position in its path.
    ///
    /// ## __Note__: This does __not__ check if the next position is a valid move in the map. The position
    /// could be occupied by an obstacle, our out of the map's bounds. The caller must validate this.
    pub const fn get_next_pos(&self, cur_pos: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (cur_pos.0 - 1, cur_pos.1),
            Self::Right => (cur_pos.0, cur_pos.1 + 1),
            Self::Down => (cur_pos.0 + 1, cur_pos.1),
            Self::Left => (cur_pos.0, cur_pos.1 - 1),
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

    let ans_one = part_one(&input)?;
    println!("Part one:\t{}", ans_one);

    let ans_two = part_two(input)?;
    println!("Part two:\t{}", ans_two);

    Ok(())
}

fn parse_input(raw_input: impl BufRead) -> Result<GuardMap> {
    raw_input
        .lines()
        .map(|line| Ok(line?.chars().map(Space::from).collect()))
        .collect()
}

fn part_one(input: &GuardMap) -> Result<usize> {
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
        let next_pos = guard.get_next_pos(pos);

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

fn part_two(mut input: GuardMap) -> Result<usize> {
    /// Determine if the path that the guard is currently taking is a loop
    fn check_loop(
        cur_pos: (usize, usize),
        guard: &Guard,
        visited: &HashMap<(usize, usize), HashSet<Guard>>,
    ) -> bool {
        visited
            .get(&cur_pos)
            .map(|data| data.contains(guard))
            // if cur_pos is not yet visited, this cannot be a loop
            .unwrap_or(false)
    }

    let mut loops = 0;
    // Find the starting position of the guard
    let (starting_pos, mut guard) = input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, space)| match space {
                Space::Guard(guard) => Some(((y, x), *guard)),
                _ => None,
            })
        })
        .context("No guard found in input map")?;

    // We would only try to place obstacles along the guard's path
    let mut possible_positions = HashSet::new();
    let mut pos = starting_pos;
    while (pos.0 > 0 && pos.0 < input.len() - 1) && (pos.1 > 0 && pos.1 < input[0].len() - 1) {
        let next_pos = guard.get_next_pos(pos);

        match &input[next_pos.0][next_pos.1] {
            Space::Obstacle => guard.rotate(),
            _ => {
                pos = next_pos;
                // We can't place an obstacle at the starting position
                if pos != starting_pos {
                    possible_positions.insert(pos);
                }
            }
        }
    }

    for obs_pos in possible_positions {
        // Maps the position of the guard to the orientation of the guard when it was previously at that position.
        // Used to check for loops - if the guard is at a position it already visited and in the same orientation,
        // then it's in a loop.
        let mut visited: HashMap<(usize, usize), HashSet<Guard>> = HashMap::new();
        let original_space = &mut Space::Obstacle;
        std::mem::swap(&mut input[obs_pos.0][obs_pos.1], original_space);
        input[obs_pos.0][obs_pos.1] = Space::Obstacle;
        let mut guard_pos = starting_pos;
        guard = Guard::default();

        while (guard_pos.0 > 0 && guard_pos.0 < input.len() - 1)
            && (guard_pos.1 > 0 && guard_pos.1 < input[0].len() - 1)
        {
            let next_pos = guard.get_next_pos(guard_pos);

            match &input[next_pos.0][next_pos.1] {
                Space::Obstacle => guard.rotate(),
                _ => {
                    visited.entry(guard_pos).or_default().insert(guard);
                    guard_pos = next_pos;
                }
            }

            if check_loop(guard_pos, &guard, &visited) {
                loops += 1;
                break;
            }
        }
        std::mem::swap(&mut input[obs_pos.0][obs_pos.1], original_space);
    }

    Ok(loops)
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

        let actual = part_one(&input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 6;
        let input = parse_input(TEST_INPUT.as_bytes()).unwrap();

        let actual = part_two(input).unwrap();
        assert_eq!(expected, actual);
    }
}
