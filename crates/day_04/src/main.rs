use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::Result;

const INPUT_FILE: &str = "input.txt";

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
const WORD_REV: [char; 4] = [WORD[3], WORD[2], WORD[1], WORD[0]];

type Matrix<T> = Vec<Vec<T>>;

fn main() {
    if let Err(e) = runner() {
        eprintln!("ERROR: {}", e);
    }
}

fn runner() -> Result<()> {
    println!("===== DAY 04 =====");

    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(INPUT_FILE);
    let input_reader = BufReader::new(File::open(input_path)?);
    let input_data = parse_input(input_reader)?;

    let ans_one = part_one(&input_data);
    println!("Part one:\t{}", ans_one);

    Ok(())
}

fn parse_input(input_reader: impl BufRead) -> Result<Matrix<char>> {
    input_reader
        .lines()
        .try_fold(Vec::new(), |mut acc, line| -> Result<Vec<_>> {
            let row: Vec<_> = line?.chars().collect();
            acc.push(row);
            Ok(acc)
        })
}

fn part_one<R, C>(input: R) -> usize
where
    R: AsRef<[C]>,
    C: AsRef<[char]>,
{
    let mut count = 0;
    let grid = input.as_ref();
    for (y, row) in input.as_ref().iter().map(|r| r.as_ref()).enumerate() {
        for (x, el) in row.iter().enumerate() {
            if WORD[0].eq(el) || WORD[WORD.len() - 1].eq(el) {
                let horiz = row.get(x..x + WORD.len());

                let vert = (|| -> Option<_> {
                    let first = grid.get(y).and_then(|row| row.as_ref().get(x))?;
                    let second = grid.get(y + 1).and_then(|row| row.as_ref().get(x))?;
                    let third = grid.get(y + 2).and_then(|row| row.as_ref().get(x))?;
                    let fourth = grid.get(y + 3).and_then(|row| row.as_ref().get(x))?;
                    Some([*first, *second, *third, *fourth])
                })();
                let vert = vert.as_ref().map(|x| x.as_slice());

                let diag_right = (|| -> Option<_> {
                    let first = grid.get(y).and_then(|row| row.as_ref().get(x))?;
                    let second = grid.get(y + 1).and_then(|row| row.as_ref().get(x + 1))?;
                    let third = grid.get(y + 2).and_then(|row| row.as_ref().get(x + 2))?;
                    let fourth = grid.get(y + 3).and_then(|row| row.as_ref().get(x + 3))?;
                    Some([*first, *second, *third, *fourth])
                })();
                let diag_right = diag_right.as_ref().map(|x| x.as_slice());

                let diag_left = (|| -> Option<_> {
                    let first = grid.get(y).and_then(|row| row.as_ref().get(x))?;
                    let second = grid
                        .get(y + 1)
                        .and_then(|row| row.as_ref().get(x.checked_sub(1)?))?;
                    let third = grid
                        .get(y + 2)
                        .and_then(|row| row.as_ref().get(x.checked_sub(2)?))?;
                    let fourth = grid
                        .get(y + 3)
                        .and_then(|row| row.as_ref().get(x.checked_sub(3)?))?;
                    Some([*first, *second, *third, *fourth])
                })();
                let diag_left = diag_left.as_ref().map(|x| x.as_slice());

                let matches = [horiz, vert, diag_right, diag_left]
                    .into_iter()
                    .filter(|word| word.is_some_and(|w| w == &WORD || w == &WORD_REV));

                count += matches.count();
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        let expected = 18;

        let data = parse_input(INPUT.as_bytes()).unwrap();
        let actual = part_one(data);

        assert_eq!(expected, actual);
    }
}
